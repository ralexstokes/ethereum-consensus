mod template;

use convert_case::{Case, Casing};
use std::{
    collections::HashMap,
    fmt::Write,
    fs,
    path::{Component, Path, PathBuf},
};
use template::{build_index, TestIndex};
use walkdir::{DirEntry, Error, WalkDir};

const COMPONENTS_IN_TEST_ROOT: usize = 2;

fn component_to_string(c: Component) -> String {
    let c = c.as_os_str().to_str().unwrap().to_string();
    c.to_case(Case::Snake)
}

fn insert_test(tests: &mut TestIndex, path: &Path) -> bool {
    let mut components = path.components().skip(COMPONENTS_IN_TEST_ROOT);

    let config = component_to_string(components.next().unwrap());
    let tests = tests.entry(config).or_default();

    let fork = component_to_string(components.next().unwrap());
    let tests = tests.entry(fork).or_default();

    let runner = component_to_string(components.next().unwrap());

    // NOTE: these tests are handled elsewhere outside this repo.
    if runner == "ssz_generic" {
        return false
    }

    let tests = tests.entry(runner).or_default();

    let handler = component_to_string(components.next().unwrap());
    let tests = tests.entry(handler).or_default();

    let suite = component_to_string(components.next().unwrap());
    let tests = tests.entry(suite).or_default();

    let test_case = component_to_string(components.next().unwrap());

    // NOTE: account for the fact that we run the tests from a nested crate relative to the test
    // data
    let path = path.display();
    let from_crate_root = format!("../{path}");
    tests.insert(test_case, from_crate_root);

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
        let module = if module == "phase_0" { "phase0".to_string() } else { module };
        writeln!(src, "mod {module};").unwrap();
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
    WalkDir::new(path).into_iter().filter(|e| is_leaf(e.as_ref().unwrap())).map(|e| e.unwrap())
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
    let mut src = r"// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

"
    .to_string();

    writeln!(src, "use crate::spec_test_runners::{runner}::{test_case_type};",).unwrap();

    let needs_trait_import = matches!(runner, "bls");

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
        fork => unimplemented!("fork {fork} not supported"),
    };
    if let Some(handler_data) = auxilliary_data.get(runner) {
        if let Some(data) = handler_data.get(handler) {
            let mut generics = data.test_case_type_generics.to_string();
            // NOTE: special case for this handler...
            if runner == "genesis" && handler == "initialization" {
                if spec == Spec::Bellatrix {
                    generics += ", spec::ExecutionPayloadHeader"
                } else {
                    // "placeholder" type, can be whatever
                    // ideally would  use `()` but it does not implement `ssz_rs::Deserialize`
                    generics += ", usize"
                }
            }
            test_case_type = format!("{test_case_type}::<{generics}>");
            if data.execution_handler.len() == 1 && data.execution_handler.contains_key(&Spec::All)
            {
                execution_handler += &data.execution_handler[&Spec::All];
            } else {
                assert!(data.execution_handler.len() > 1);
                let handler = &data.execution_handler[&spec];
                execution_handler += handler;
            }
            if !matches!(runner, "ssz_static" | "rewards" | "fork") {
                mut_decl += "mut";
            }
            if let Some(s) = data.preamble.get(&spec) {
                preamble += s;
            }
            if let Some(s) = data.preamble.get(&Spec::All) {
                preamble += s;
            }
            needs_spec_import = true;
        }
    }

    if execution_handler.is_empty() {
        execution_handler += "execute()";
    }

    if needs_spec_import {
        writeln!(src, "use ethereum_consensus::{fork}::{config} as spec;").unwrap();
    }

    // special case imports here...
    match runner {
        "ssz_static" => writeln!(src, "use ethereum_consensus::ssz::prelude::*;").unwrap(),
        "transition" => {
            writeln!(
                src,
                "use ethereum_consensus::state_transition::{config}::{{Executor, BeaconState}};",
            )
            .unwrap();
            let pre_fork = match spec {
                Spec::Altair => "phase0",
                Spec::Bellatrix => "altair",
                _ => unimplemented!("support other forks"),
            };
            writeln!(src, "use ethereum_consensus::{pre_fork}::{config} as pre_spec;",).unwrap();
        }
        "fork" => match spec {
            Spec::Altair => {
                writeln!(src, "use ethereum_consensus::phase0::{config} as phase0;").unwrap();
            }
            Spec::Bellatrix => {
                writeln!(src, "use ethereum_consensus::altair::{config} as altair;").unwrap();
            }
            _ => todo!("support other forks"),
        },
        _ => {}
    }

    let mut test_cases = tests.keys().cloned().collect::<Vec<_>>();
    test_cases.sort();
    for test_case in &test_cases {
        let source_path = &tests[test_case];
        let test_src = format!(
            r#"
#[test]
fn test_{test_case}() {{
    let {mut_decl} test_case = {test_case_type}::from("{source_path}");
    {preamble}
    test_case.{execution_handler};
}}
"#
        );
        src += &test_src;
    }

    src
}

pub struct Auxillary {
    test_case_type_generics: String,
    preamble: HashMap<Spec, String>,
    execution_handler: HashMap<Spec, String>,
}

#[derive(Hash, PartialEq, Eq)]
pub enum Spec {
    Phase0,
    Altair,
    Bellatrix,
    All,
}

// we follow the pattern of the `consensus-spec-tests` that repeats names along the test hierarchy.
// clippy does not like this so we silence this lint across the entire generated set of modules.
fn fix_clippy_lint(s: &mut String) {
    let fix = r"
// Submodules here follow the layout of the spec tests for consistency
// but we must disable a clippy lint to satisfy the linter on the naming
#![allow(clippy::module_inception)]
    ";
    s.insert_str(0, fix)
}

fn main() {
    let auxilliary_data = build_index();

    let test_root = "consensus-spec-tests/tests";
    println!("discovering tests in {test_root}...");
    let (all_test_cases, collected_test_case_count) = collect_test_cases(test_root).unwrap();

    println!("collected {collected_test_case_count} tests");

    let generated_test_root = PathBuf::from("ethereum-consensus/tests/consensus_spec_tests");
    fs::remove_dir_all(&generated_test_root).unwrap();

    fs::create_dir_all(&generated_test_root).unwrap();
    let mut module_decl_src = generate_module_decl_src(all_test_cases.keys());
    fix_clippy_lint(&mut module_decl_src);
    let mut mod_path = generated_test_root.clone();
    mod_path.push("mod.rs");
    fs::write(mod_path, module_decl_src).unwrap();
    for (config, tests) in all_test_cases {
        println!("generating tests for config: {config}");
        let config = config.to_case(Case::Snake);
        let target = generated_test_root.join(&config);
        fs::create_dir_all(&target).unwrap();
        let module_decl_src = generate_module_decl_src(tests.keys());
        let mut mod_path = target.clone();
        mod_path.push("mod.rs");
        fs::write(mod_path, module_decl_src).unwrap();
        for (fork, tests) in tests {
            println!("> generating tests for fork: {fork}");
            let fork = fork.to_case(Case::Camel);
            let target = target.join(&fork);
            fs::create_dir(&target).unwrap();
            let module_decl_src = generate_module_decl_src(tests.keys());
            let mut mod_path = target.clone();
            mod_path.push("mod.rs");
            fs::write(mod_path, module_decl_src).unwrap();
            for (runner, tests) in tests {
                println!(">> generating tests for runner: {runner}");
                let runner = runner.to_case(Case::Snake);
                let target = target.join(&runner);
                fs::create_dir(&target).unwrap();
                let module_decl_src = generate_module_decl_src(tests.keys());
                let mut mod_path = target.clone();
                mod_path.push("mod.rs");
                fs::write(mod_path, module_decl_src).unwrap();
                for (handler, tests) in tests {
                    println!(">>> generating tests for handler: {handler}");
                    let handler = handler.to_case(Case::Snake);
                    let target = target.join(&handler);
                    fs::create_dir(&target).unwrap();
                    let module_decl_src = generate_module_decl_src(tests.keys());
                    let mut mod_path = target.clone();
                    mod_path.push("mod.rs");
                    fs::write(mod_path, module_decl_src).unwrap();
                    for (suite, tests) in tests {
                        println!(">>>> generating tests for suite: {suite}");
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
