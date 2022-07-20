use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::{Component, Path, PathBuf};
use walkdir::{DirEntry, Error, WalkDir};

type TestIndex = HashMap<
    String,
    HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, String>>>>>,
>;

fn component_to_string(c: Component) -> String {
    let c = c.as_os_str().to_str().unwrap().to_string();
    c.to_case(Case::Snake)
}

fn insert_test(tests: &mut TestIndex, path: &Path) -> bool {
    let mut components = path.components().skip(2);

    let config = component_to_string(components.next().unwrap());
    let tests = tests.entry(config).or_default();

    let fork = component_to_string(components.next().unwrap());
    let tests = tests.entry(fork).or_default();

    let runner = component_to_string(components.next().unwrap());

    // NOTE: these tests are handled elsewhere outside this repo.
    if runner == "ssz_generic" {
        return false;
    }

    let tests = tests.entry(runner).or_default();

    let handler = component_to_string(components.next().unwrap());
    let tests = tests.entry(handler).or_default();

    let suite = component_to_string(components.next().unwrap());
    let tests = tests.entry(suite).or_default();

    let test_case = component_to_string(components.next().unwrap());
    tests.insert(test_case, path.display().to_string());

    true
}

fn collect_test_cases(root: &str) -> Result<(TestIndex, usize), Error> {
    let mut total_test_case_count = 0;
    let mut tests = TestIndex::new();
    for entry in leaf_dirs(&PathBuf::from(root)) {
        let inserted = insert_test(&mut tests, entry.path());
        if inserted {
            total_test_case_count += 1;
        }
    }
    Ok((tests, total_test_case_count))
}

fn prefix_if_starts_with_reserved(s: &str) -> String {
    if s.as_bytes()[0].is_ascii_digit() || s == "true" || s == "false" {
        String::from("mod_") + s
    } else {
        s.to_string()
    }
}

fn generate_module_decl_src<'a>(modules: impl Iterator<Item = &'a String>) -> String {
    let mut src = String::new();
    for module in modules {
        let module = prefix_if_starts_with_reserved(module);
        let module = if module == "phase_0" {
            "phase0".to_string()
        } else {
            module
        };
        writeln!(src, "mod {};", module).unwrap();
    }
    src
}

fn is_leaf(entry: &DirEntry) -> bool {
    let mut has_children = false;
    let mut is_empty = true;

    for child in WalkDir::new(entry.path()).min_depth(1).max_depth(2) {
        is_empty = false;
        let child = child.unwrap();
        if child.metadata().unwrap().is_dir() {
            has_children = true;
        }
    }

    !has_children && !is_empty
}

fn leaf_dirs(path: &PathBuf) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter(|e| is_leaf(e.as_ref().unwrap()))
        .map(|e| e.unwrap())
}

fn generate_suite_src(
    tests: &HashMap<String, String>,
    fork: &str,
    config: &str,
    runner: &str,
    handler: &str,
    auxilliary_data: &HashMap<&str, HashMap<&str, Auxillary>>,
) -> String {
    let mut test_case_type = handler.to_case(Case::UpperCamel) + "TestCase";
    let mut src = format!(
        r#"// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::{}::{};
"#,
        runner, test_case_type
    );

    let needs_trait_import = !matches!(runner, "sanity");

    if needs_trait_import {
        src += "use crate::test_utils::TestCase;\n";
    }

    let mut needs_spec_import = false;
    let mut execution_handler = String::new();
    let mut mut_decl = String::new();
    let mut preamble = String::new();
    let spec = match fork {
        "phase0" => Spec::Phase0,
        "altair" => Spec::Altair,
        "bellatrix" => Spec::Bellatrix,
        _ => todo!("support other forks"),
    };
    if let Some(handler_data) = auxilliary_data.get(runner) {
        if let Some(data) = handler_data.get(handler) {
            test_case_type = format!("{}::<{}>", test_case_type, data.test_case_type_generics);
            if data.execution_handler.len() == 1 && data.execution_handler.contains_key(&Spec::All)
            {
                execution_handler += &data.execution_handler[&Spec::All];
            } else {
                assert!(data.execution_handler.len() > 1);
                let handler = &data.execution_handler[&spec];
                execution_handler += handler;
            }
            mut_decl += "mut";
            if let Some(s) = data.preamble.get(&spec) {
                preamble += s;
            }
            needs_spec_import = true;
        }
    }

    if execution_handler.is_empty() {
        execution_handler += "execute()";
    }

    if needs_spec_import {
        writeln!(src, "use ethereum_consensus::{}::{} as spec;", fork, config).unwrap();
    }

    let mut test_cases = tests.keys().cloned().collect::<Vec<_>>();
    test_cases.sort();
    for test_case in &test_cases {
        let source_path = &tests[test_case];
        let test_src = format!(
            r#"
#[test]
fn test_{}() {{
    let {} test_case = {}::from("{}");
    {}
    test_case.{};
}}
"#,
            test_case, mut_decl, test_case_type, source_path, preamble, execution_handler,
        );
        src += &test_src;
    }

    src
}

struct Auxillary {
    test_case_type_generics: String,
    preamble: HashMap<Spec, String>,
    execution_handler: HashMap<Spec, String>,
}

#[derive(Hash, PartialEq, Eq)]
enum Spec {
    Phase0,
    Altair,
    Bellatrix,
    All,
}

fn main() {
    let auxilliary_data = HashMap::from([
        ("sanity",
        HashMap::from([
            (
                "slots",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, offset, context| {
                    let target_slot = state.slot + offset;
                    spec::process_slots(state, target_slot, context).unwrap();
                })"
                    .to_string())]),
                },
            ),
            (
                "blocks",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedBeaconBlock"
                        .to_string(),
                    preamble: HashMap::from_iter([(Spec::Bellatrix, "let execution_engine = spec::NoOpExecutionEngine;".to_string())]),
                    execution_handler: HashMap::from_iter([(Spec::Phase0, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Altair, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Bellatrix, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, execution_engine.clone(), validation, context)?;
                        }
                        Ok(())
                    })".to_string())]),
                },
            ),
        ])),
        ("finality",
        HashMap::from([
            (
                "finality",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState, spec::SignedBeaconBlock"
                        .to_string(),
                    preamble: HashMap::from_iter([(Spec::Bellatrix, "let execution_engine = spec::NoOpExecutionEngine;".to_string())]),
                    execution_handler: HashMap::from_iter([(Spec::Phase0, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Altair, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, validation, context)?;
                        }
                        Ok(())
                    })".to_string()), (Spec::Bellatrix, "execute(|state, blocks, validation, context| {
                        for block in blocks.iter_mut() {
                            spec::state_transition(state, block, execution_engine.clone(), validation, context)?;
                        }
                        Ok(())
                    })".to_string())]),
                },
            ),
        ])),
        ("epoch_processing",
        HashMap::from([
            (
                "effective_balance_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_effective_balance_updates(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "eth_1_data_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_eth1_data_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "historical_roots_update",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_historical_roots_update(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "inactivity_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_inactivity_updates(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "justification_and_finalization",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_justification_and_finalization(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "participation_record_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_participation_record_updates(state);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "participation_flag_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_participation_flag_updates(state)
                })"
                    .to_string())]),
                },
            ),
            (
                "randao_mixes_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_randao_mixes_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "registry_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_registry_updates(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "rewards_and_penalties",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_rewards_and_penalties(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "slashings",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_slashings(state, context)
                })"
                    .to_string())]),
                },
            ),
            (
                "slashings_reset",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_slashings_reset(state, context);
                        Ok(())
                })"
                    .to_string())]),
                },
            ),
            (
                "sync_committee_updates",
                Auxillary {
                    test_case_type_generics: "spec::BeaconState".to_string(),
                    preamble: Default::default(),
                    execution_handler: HashMap::from_iter([(Spec::All, "execute(|state, context| {
                        spec::process_sync_committee_updates(state, context)
                })"
                    .to_string())]),
                },
            ),
        ])),
    ]);
    let test_root = "consensus-spec-tests/tests";
    let (all_test_cases, collected_test_case_count) = collect_test_cases(test_root).unwrap();

    println!("collected {collected_test_case_count} tests");

    let generated_test_root = PathBuf::from("tests/consensus_spec_tests");
    fs::remove_dir_all(&generated_test_root).unwrap();

    fs::create_dir_all(&generated_test_root).unwrap();
    let module_decl_src = generate_module_decl_src(all_test_cases.keys());
    let mut mod_path = generated_test_root.clone();
    mod_path.push("mod.rs");
    fs::write(mod_path, module_decl_src).unwrap();
    for (config, tests) in all_test_cases {
        let config = config.to_case(Case::Snake);
        let target = generated_test_root.join(&config);
        fs::create_dir_all(&target).unwrap();
        let module_decl_src = generate_module_decl_src(tests.keys());
        let mut mod_path = target.clone();
        mod_path.push("mod.rs");
        fs::write(mod_path, module_decl_src).unwrap();
        for (fork, tests) in tests {
            let fork = fork.to_case(Case::Camel);
            let target = target.join(&fork);
            fs::create_dir(&target).unwrap();
            let module_decl_src = generate_module_decl_src(tests.keys());
            let mut mod_path = target.clone();
            mod_path.push("mod.rs");
            fs::write(mod_path, module_decl_src).unwrap();
            for (runner, tests) in tests {
                let runner = runner.to_case(Case::Snake);
                let target = target.join(&runner);
                fs::create_dir(&target).unwrap();
                let module_decl_src = generate_module_decl_src(tests.keys());
                let mut mod_path = target.clone();
                mod_path.push("mod.rs");
                fs::write(mod_path, module_decl_src).unwrap();
                for (handler, tests) in tests {
                    let handler = handler.to_case(Case::Snake);
                    let target = target.join(&handler);
                    fs::create_dir(&target).unwrap();
                    let module_decl_src = generate_module_decl_src(tests.keys());
                    let mut mod_path = target.clone();
                    mod_path.push("mod.rs");
                    fs::write(mod_path, module_decl_src).unwrap();
                    for (suite, tests) in tests {
                        let suite = suite.to_case(Case::Snake);
                        let target = target.join(suite);
                        fs::create_dir(&target).unwrap();
                        let mut mod_path = target.clone();
                        mod_path.push("mod.rs");
                        // NOTE: batch test cases to reduce overall file count
                        // which strains git, IDEs, etc.
                        let suite_src = generate_suite_src(
                            &tests,
                            &fork,
                            &config,
                            &runner,
                            &handler,
                            &auxilliary_data,
                        );
                        fs::write(mod_path, suite_src).unwrap();
                    }
                }
            }
        }
    }
}
