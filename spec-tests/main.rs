/// Entrypoint for running the spec tests
/// Uses a custom test harness via `libtest_mimic` to dynamically discover tests to execute
use ethereum_consensus::state_transition;
use libtest_mimic::{Arguments, Trial};
use std::{
    env,
    error::Error,
    fs::{self, DirEntry},
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use test_case::*;
use test_meta::*;

mod runners;
mod test_case;
mod test_meta;
mod test_utils;

const SPEC_TESTS_ROOT_PATH: &str = "consensus-spec-tests/tests";

fn str_from_path_component<'a>(component: &'a Component) -> &'a str {
    component.as_os_str().to_str().unwrap()
}

fn parse_test_meta(parent: &Path, path_mask: &PathBuf) -> TestMeta {
    let target = parent.strip_prefix(path_mask).unwrap();
    let mut components = target.components();
    let config = Config::from(str_from_path_component(components.next().as_ref().unwrap()));
    let fork = Fork::from(str_from_path_component(components.next().as_ref().unwrap()));
    let runner = Runner::from(str_from_path_component(components.next().as_ref().unwrap()));
    let handler = Handler::from(str_from_path_component(components.next().as_ref().unwrap()));
    let suite = Suite::from(str_from_path_component(components.next().as_ref().unwrap()));
    let case = Case::from(str_from_path_component(components.next().as_ref().unwrap()));
    assert!(components.next().is_none());
    TestMeta { config, fork, runner, handler, suite, case }
}

fn parse_test_case(
    test_data_paths: &[PathBuf],
    path_mask: &PathBuf,
    context: Arc<Context>,
) -> TestCase {
    let parent = test_data_paths.first().expect("some test data").parent().expect("has parent");
    let test_meta = parse_test_meta(parent, path_mask);
    TestCase::new(test_meta, parent, context)
}

fn has_children(dir_entries: &[DirEntry]) -> bool {
    dir_entries.iter().any(|entry| {
        let file_type = entry.file_type().unwrap();
        file_type.is_dir()
    })
}

fn visit_dir(
    path: &Path,
    tests: &mut Vec<Trial>,
    path_mask: &PathBuf,
    context: Arc<Context>,
) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, _>>()?;
    if has_children(&entries) {
        for entry in entries {
            assert!(entry.file_type()?.is_dir());
            let path = entry.path();
            visit_dir(&path, tests, path_mask, context.clone())?;
        }
    } else {
        if entries.is_empty() {
            // some spurious test dirs exist
            return Ok(())
        }
        let test_case_paths = entries
            .iter()
            .map(|entry| {
                assert!(entry.file_type().unwrap().is_file());
                entry.path()
            })
            .collect::<Vec<_>>();
        let same_parents = test_case_paths.windows(2).all(|w| {
            let x = w[0].parent();
            let y = w[1].parent();
            x == y
        });
        assert!(same_parents);

        let test_case = parse_test_case(&test_case_paths, path_mask, context.clone());
        if test_case.meta.should_skip() {
            return Ok(())
        }
        let name = test_case.name();
        let should_ignore = test_case.meta.should_ignore();
        let test = Trial::test(name, move || test_case.execute().map_err(From::from))
            .with_ignored_flag(should_ignore);
        tests.push(test);
    }

    Ok(())
}

fn collect_tests<P: AsRef<Path>>(
    spec_tests_root: P,
    context: Arc<Context>,
) -> Result<Vec<Trial>, Box<dyn Error>> {
    let mut tests = Vec::new();
    let mut root_dir = env::current_dir()?;
    root_dir.push(spec_tests_root.as_ref());
    let path_mask = root_dir.to_path_buf();
    visit_dir(&root_dir, &mut tests, &path_mask, context)?;
    Ok(tests)
}

struct Context {
    mainnet: state_transition::Context,
    minimal: state_transition::Context,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::from_args();
    let context = Arc::new(Context {
        mainnet: state_transition::Context::for_mainnet(),
        minimal: state_transition::Context::for_minimal(),
    });
    let tests = collect_tests(SPEC_TESTS_ROOT_PATH, context)?;
    libtest_mimic::run(&args, tests).exit()
}
