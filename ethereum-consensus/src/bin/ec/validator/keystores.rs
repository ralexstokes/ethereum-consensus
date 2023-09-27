use bip39::Mnemonic;
use ethereum_consensus::crypto::hash;
use hkdf::Hkdf;
use rayon::prelude::*;
use ruint::{aliases::U256, uint, Uint};
use sha2::Sha256;
use thiserror::Error;

type U381 = Uint<381, 6>;

const SALT: &[u8; 20] = b"BLS-SIG-KEYGEN-SALT-";
const L: u8 = 48;
const R: U381 =
    uint!(52435875175126190479447740508185965837690552500527637822603658699938581184513_U381);

#[derive(Debug, Error)]
pub enum Error {}

#[derive(Debug, Default)]
struct Key(U256);

#[derive(Debug)]
pub struct Keystore {
    key: Key,
}

fn ikm_to_lamport_secret_key(ikm: &[u8], salt: &[u8]) -> [[u8; 32]; 255] {
    Default::default()
}

fn parent_key_to_lamport_public_key(key: &Key, index: u32) -> [u8; 32] {
    let salt = index.to_be_bytes();
    let ikm: [u8; 32] = key.0.to_be_bytes();
    let lamport_0 = ikm_to_lamport_secret_key(&ikm, &salt);
    let not_ikm = flip_bits(&ikm);
    let lamport_1 = ikm_to_lamport_secret_key(&not_ikm, &salt);
    let lamport_public_key = Vec::with_capacity(8160);
    for i in 1..=255 {
        //
    }
    for i in 1..=255 {
        //
    }
    let compressed_lamport_public_key = hash(&lamport_public_key);
    compressed_lamport_public_key.as_ref().try_into().unwrap()
}

fn hkdf_mod_r(input: &[u8]) -> Key {
    let mut key = U381::ZERO;
    let mut salt = hash(SALT);
    let key_info = [0, L];
    let mut ikm = input.to_vec();
    ikm.push(0);

    while key == U381::ZERO {
        let hk = Hkdf::<Sha256>::new(Some(salt.as_ref()), &ikm);
        let mut okm = [0u8; 48];
        hk.expand(&key_info, &mut okm).expect("length L is valid");
        let inner = U381::from_be_bytes(okm);
        key = inner % R;

        salt = hash(&salt);
    }

    let key_bytes: [u8; 48] = key.to_be_bytes();
    let inner: [u8; 32] = key_bytes[16..].try_into().unwrap();
    Key(U256::from_be_bytes(inner))
}

fn derive_child_key(parent_key: &Key, index: u32) -> Key {
    let compressed_lamport_public_key = parent_key_to_lamport_public_key(parent_key, index);
    hkdf_mod_r(&compressed_lamport_public_key)
}

fn derive_master_sk(mnemonic: Mnemonic) -> Key {
    let seed = mnemonic.to_seed("");
    hkdf_mod_r(&seed)
}

fn derive_keystore(root_key: &Key, index: u32) -> Result<Keystore, Error> {
    let child_key = derive_child_key(root_key, index);
    Ok(Keystore { key: child_key })
}

pub fn generate(mnemonic: Mnemonic, start: u32, end: u32) -> Result<Vec<Keystore>, Error> {
    let root_key = derive_master_sk(mnemonic);
    (start..end)
        .into_par_iter()
        .map(|i| derive_keystore(&root_key, i))
        .collect::<Result<Vec<_>, _>>()
}
