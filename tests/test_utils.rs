use glob::glob;
use hex::FromHexError;
use serde::{de::Error, Deserialize, Deserializer};
use std::fs::File;

const YAML_NULL_CHAR: &str = "~";

pub trait TestCase
where
    Self: for<'de> serde::Deserialize<'de>,
{
    fn execute(&self);

    fn execute_from_glob(path_glob: &str) {
        let entries = glob(path_glob).expect("Failed to read glob pattern");
        for entry in entries {
            let path = entry.unwrap();
            let file = File::open(path).expect("File does not exist");
            let test_case: Self =
                serde_yaml::from_reader(file).expect("Is not well-formatted yaml");
            test_case.execute()
        }
    }
}

fn hex_from_string(data: &str) -> Result<Vec<u8>, FromHexError> {
    let data = data.as_bytes();
    let data = if data.starts_with(b"0x") {
        &data[2..]
    } else {
        data
    };
    hex::decode(data)
}

pub fn decode_hex_with_prefix<'a, D>(input: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: String = Deserialize::deserialize(input)?;
    hex_from_string(&data).map_err(D::Error::custom)
}

pub fn decode_hex_with_prefix_maybe<'a, D>(input: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: String = Deserialize::deserialize(input)?;
    if data == YAML_NULL_CHAR {
        return Ok(None);
    }
    hex_from_string(&data).map(Some).map_err(D::Error::custom)
}

pub fn decode_hex_vec_with_prefix<'a, D>(input: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: Vec<String> = Deserialize::deserialize(input)?;
    if data.is_empty() {
        return Ok(vec![]);
    }
    let data = data
        .iter()
        .map(|x| {
            if x == YAML_NULL_CHAR {
                vec![]
            } else {
                hex_from_string(x).unwrap()
            }
        })
        .collect();
    Ok(data)
}
