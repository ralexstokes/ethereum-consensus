use bip39::Mnemonic;
use ethereum_consensus::crypto::hash;
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

#[derive(Debug, Default)]
pub struct Key(U256);

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

        salt = hash(&salt);
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

fn derive_master_sk(mnemonic: Mnemonic) -> Key {
    let seed = mnemonic.to_seed("");
    hkdf_mod_r(&seed)
}

fn derive_keystore(root_key: &Key, index: u32) -> Key {
    derive_child_key(root_key, index)
}

pub fn generate(mnemonic: Mnemonic, start: u32, end: u32) -> Vec<Key> {
    let root_key = derive_master_sk(mnemonic);
    (start..end).into_par_iter().map(|i| derive_keystore(&root_key, i)).collect()
}
