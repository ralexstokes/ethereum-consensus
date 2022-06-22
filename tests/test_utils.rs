#![cfg(feature = "spec-tests")]

use serde::Deserialize;
use std::fmt;
use std::fs::File;

pub trait TestCase: fmt::Debug {
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
}

pub fn load_yaml<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let mut file = File::open(&path).expect("File does not exist");
    let test_case: Result<T, _> = serde_yaml::from_reader(&mut file);
    match test_case {
        Ok(test_case) => test_case,
        Err(err) => {
            let content = std::fs::read_to_string(&path).unwrap();
            panic!("{err} from {content} at {path:?}")
        }
    }
}
