use ethereum_consensus::Error as SpecError;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown handler `{0}` in `{1}`")]
    UnknownHandler(String, String),
    #[error(transparent)]
    Spec(#[from] SpecError),
    #[error("state did not match expected")]
    InvalidState,
    #[error("test was expected to error but did not")]
    ExpectedError,
    #[error("internal error; you should not see this")]
    InternalContinue,
}

pub fn load_yaml<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let mut file = File::open(path).expect("File does not exist");
    let test_case: Result<T, _> = serde_yaml::from_reader(&mut file);
    match test_case {
        Ok(test_case) => test_case,
        Err(err) => {
            let content = std::fs::read_to_string(path).unwrap();
            panic!("{err} from {content} at {path:?}")
        }
    }
}

pub fn load_snappy_ssz_bytes(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut data = vec![];
    file.read_to_end(&mut data).unwrap();

    let mut decoder = snap::raw::Decoder::new();
    decoder.decompress_vec(&data).unwrap()
}

pub fn load_snappy_ssz<T: ssz_rs::Deserialize>(path: &str) -> Option<T> {
    let path = Path::new(path);
    if !path.exists() {
        return None
    }
    let buffer = load_snappy_ssz_bytes(path);

    let result = <T as ssz_rs::Deserialize>::deserialize(&buffer);

    result.ok()
}
