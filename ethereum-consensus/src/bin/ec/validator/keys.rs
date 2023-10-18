use crate::validator::mnemonic::Seed;
use ethereum_consensus::crypto::{hash, PublicKey as BlsPublicKey, SecretKey as BlsSecretKey};
use hkdf::Hkdf;
use rayon::prelude::*;
use ruint::{aliases::U256, uint, Uint};
use sha2::Sha256;

type U384 = Uint<384, 6>;

const SALT: &[u8; 20] = b"BLS-SIG-KEYGEN-SALT-";
const L: usize = 48;
const R: U384 =
    uint!(52435875175126190479447740508185965837690552500527637822603658699938581184513_U384);
const K: usize = 32;
const LAMPORT_COUNT: usize = 255;
const LAMPORT_L: usize = K * LAMPORT_COUNT;

pub type Path = String;

#[derive(Debug)]
pub struct KeyPair {
    pub private_key: BlsSecretKey,
    pub public_key: BlsPublicKey,
    pub path: Path,
}

// (signing, withdrawal)
pub type ValidatorKeys = (KeyPair, KeyPair);

#[derive(Debug, Default, Clone)]
struct Key(U256);

fn bytes_split<const M: usize, const N: usize>(input: &[u8]) -> Vec<&[u8]> {
    debug_assert!(M % N == 0);
    debug_assert!(input.len() == M);

    input.chunks_exact(N).collect()
}

fn ikm_to_lamport_secret_key<'a>(ikm: &[u8], salt: &[u8], output: &'a mut [u8]) -> Vec<&'a [u8]> {
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
    hk.expand(&[], output).expect("length L is valid");
    bytes_split::<LAMPORT_L, K>(output)
}

fn flip_bits(input: [u8; 32]) -> Vec<u8> {
    input.into_iter().map(|i| !i).collect()
}

fn parent_key_to_lamport_public_key(key: &Key, index: u32) -> [u8; 32] {
    let salt = index.to_be_bytes();
    let ikm: [u8; 32] = key.0.to_be_bytes();
    let mut output_0 = [0u8; LAMPORT_L];
    let lamport_0 = ikm_to_lamport_secret_key(&ikm, &salt, &mut output_0);
    let not_ikm = flip_bits(ikm);
    let mut output_1 = [0u8; LAMPORT_L];
    let lamport_1 = ikm_to_lamport_secret_key(&not_ikm, &salt, &mut output_1);
    let mut lamport_public_key = Vec::with_capacity(LAMPORT_L);
    lamport_0.into_iter().map(hash).for_each(|data| {
        lamport_public_key.extend_from_slice(data.as_ref());
    });
    lamport_1.into_iter().map(hash).for_each(|data| {
        lamport_public_key.extend_from_slice(data.as_ref());
    });
    debug_assert_eq!(lamport_public_key.len(), 16320);
    let compressed_lamport_public_key = hash(&lamport_public_key);
    compressed_lamport_public_key.as_ref().try_into().unwrap()
}

fn hkdf_mod_r(input: &[u8]) -> Key {
    let mut key = U384::ZERO;
    let mut salt = hash(SALT);
    // safety: `L` fits in type by definition
    let l = L as u8;
    let key_info = [0, l];
    let mut ikm = input.to_vec();
    ikm.push(0);

    while key == U384::ZERO {
        let hk = Hkdf::<Sha256>::new(Some(salt.as_ref()), &ikm);
        let mut okm = [0u8; L];
        hk.expand(&key_info, &mut okm).expect("length L is valid");
        let inner = U384::from_be_bytes(okm);
        key = inner % R;

        salt = hash(salt.as_ref());
    }

    // ensure we are in the field
    debug_assert_eq!(key % R, key);

    let key_bytes: [u8; L] = key.to_be_bytes();
    let inner: [u8; 32] = key_bytes[16..].try_into().unwrap();
    Key(U256::from_be_bytes(inner))
}

fn derive_child_key(parent_key: &Key, index: u32) -> Key {
    let compressed_lamport_public_key = parent_key_to_lamport_public_key(parent_key, index);
    hkdf_mod_r(&compressed_lamport_public_key)
}

fn derive_master_sk(seed: &Seed) -> Key {
    hkdf_mod_r(seed)
}

fn to_bls_secret_key(key: Key) -> BlsSecretKey {
    let key: [u8; 32] = key.0.to_be_bytes();
    BlsSecretKey::try_from(key.as_ref()).unwrap()
}

fn to_key_pair(key: Key, path: Path) -> KeyPair {
    let private_key = to_bls_secret_key(key);
    let public_key = private_key.public_key();
    KeyPair { private_key, public_key, path }
}

fn derive_validator_keys(root_key: &Key, index: u32) -> ValidatorKeys {
    // NOTE: hard-coded path for these keys following EIP-2334
    let withdrawal_key = [12381, 3600, index, 0]
        .into_iter()
        .fold(root_key.clone(), |key, index| derive_child_key(&key, index));
    let signing_key = derive_child_key(&withdrawal_key, 0);

    let signing = to_key_pair(signing_key, format!("m/12381/3600/{index}/0/0"));
    let withdrawal = to_key_pair(withdrawal_key, format!("m/12381/3600/{index}/0"));
    (signing, withdrawal)
}

pub fn generate(seed: &Seed, start: u32, end: u32) -> (Vec<KeyPair>, Vec<KeyPair>) {
    let root_key = derive_master_sk(seed);
    (start..end).into_par_iter().map(|i| derive_validator_keys(&root_key, i)).unzip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::mnemonic;

    #[test]
    fn test_simple_key_derive() {
        let mnemonic = mnemonic::recover_from_phrase("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
        let passphrase = "TREZOR";
        let seed = mnemonic.to_seed(passphrase);
        let expected_seed = [
            197, 82, 87, 195, 96, 192, 124, 114, 2, 154, 235, 193, 181, 60, 5, 237, 3, 98, 173,
            163, 142, 173, 62, 62, 158, 250, 55, 8, 229, 52, 149, 83, 31, 9, 166, 152, 117, 153,
            209, 130, 100, 193, 225, 201, 47, 44, 241, 65, 99, 12, 122, 60, 74, 183, 200, 27, 47,
            0, 22, 152, 231, 70, 59, 4,
        ];
        assert_eq!(seed, expected_seed);

        let root_key = derive_master_sk(&seed);
        let expected_root_key = uint!(
            6083874454709270928345386274498605044986640685124978867557563392430687146096_U256
        );
        assert_eq!(root_key.0, expected_root_key);

        let child_index = 0;
        let child_key = derive_child_key(&root_key, child_index);
        let expected_child_key = uint!(
            20397789859736650942317412262472558107875392172444076792671091975210932703118_U256
        );
        assert_eq!(child_key.0, expected_child_key);
    }
}
