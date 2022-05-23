use glob::glob;
use serde::Deserialize;
use std::fmt;
use std::fs::File;

pub trait TestCase
where
    Self: for<'de> Deserialize<'de> + fmt::Debug,
{
    fn should_succeed(&self) -> bool;

    fn verify_success(&self) -> bool;

    fn verify_failure(&self) -> bool;

    fn execute(&self) {
        let result = if self.should_succeed() {
            self.verify_success()
        } else {
            self.verify_failure()
        };
        assert!(result, "{:#?}", self)
    }

    fn execute_from_glob(path_glob: &str) {
        let entries = glob(path_glob).expect("Failed to read glob pattern");
        for entry in entries {
            let path = entry.unwrap();
            let mut file = File::open(&path).expect("File does not exist");
            let test_case: Result<Self, _> = serde_yaml::from_reader(&mut file);
            match test_case {
                Ok(test_case) => test_case.execute(),
                Err(err) => {
                    let content = std::fs::read_to_string(&path).unwrap();
                    panic!("{err} from {content} at {path:?}")
                }
            }
        }
    }
}
