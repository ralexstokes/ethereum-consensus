use glob::glob;
use std::fs::File;

pub trait TestCase
where
    Self: for<'de> serde::Deserialize<'de>,
{
    fn execute(self);

    fn execute_from_glob(path_glob: &str) {
        let entries = glob(path_glob).expect("Failed to read glob pattern");
        for entry in entries {
            let path = entry.unwrap();
            let mut file = File::open(path).expect("File does not exist");
            let test_case: Self =
                serde_yaml::from_reader(&mut file).expect("Is not well-formatted yaml");
            test_case.execute()
        }
    }
}
