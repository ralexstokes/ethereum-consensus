use convert_case::{Case, Casing};
use std::collections::HashMap;
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
        src += &format!("mod {};\n", module);
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

fn generate_suite_src(tests: &HashMap<String, String>, runner: &str, handler: &str) -> String {
    let handler_type = handler.to_case(Case::UpperCamel) + "Handler";
    let mut src = format!(
        r#"// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::{}::{} as TestRunner;
use crate::test_utils::TestCase;

"#,
        runner, handler_type
    );

    for (test_case, source_path) in tests {
        let test_src = format!(
            r#"
#[test]
fn test_{}() {{
    let test_case = TestRunner::from("{}");
    test_case.execute();
}}
"#,
            test_case, source_path
        );
        src += &test_src;
    }

    src
}

fn main() {
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
        let target = generated_test_root.join(config);
        fs::create_dir_all(&target).unwrap();
        let module_decl_src = generate_module_decl_src(tests.keys());
        let mut mod_path = target.clone();
        mod_path.push("mod.rs");
        fs::write(mod_path, module_decl_src).unwrap();
        for (fork, tests) in tests {
            let fork = fork.to_case(Case::Snake);
            let target = target.join(fork);
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
                        let suite_src = generate_suite_src(&tests, &runner, &handler);
                        fs::write(mod_path, suite_src).unwrap();
                    }
                }
            }
        }
    }
}
