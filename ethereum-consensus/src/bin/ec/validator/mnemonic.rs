use bip39::{Error, Mnemonic};
use rand_core::{OsRng, RngCore};
use std::str::FromStr;

const KEY_SIZE: usize = 32;

pub fn generate_random_from_system_entropy() -> Result<Mnemonic, Error> {
    let mut entropy = [0u8; KEY_SIZE];
    OsRng.fill_bytes(&mut entropy);

    Mnemonic::from_entropy(&entropy)
}

pub fn recover_from_phrase(phrase: &str) -> Result<Mnemonic, Error> {
    Mnemonic::from_str(phrase)
}
