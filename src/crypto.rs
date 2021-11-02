use ssz_rs::prelude::*;
use rand::prelude::*;

use blst::{min_pk as blst_core, BLST_ERROR};

pub const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;
pub const BLS_SECRET_KEY_BYTES_LEN: usize = 32;
pub const BLS_SIGNATURE_BYTES_LEN: usize = 96;

pub type BLSPubkey = Vector<u8, BLS_PUBLIC_KEY_BYTES_LEN>;
pub type BLSSignature = Vector<u8, BLS_SIGNATURE_BYTES_LEN>;

#[derive(Debug)]
pub struct SecretKey(pub(crate) blst_core::SecretKey);

impl SecretKey {
    pub fn random() -> Self {
        let mut ikm=[0u8; 32];
        let mut rng = rand::thread_rng();
        rng.try_fill_bytes(&mut ikm).expect("unable to generate key material");
        return Self::from_bytes(&ikm)
    }

    pub fn from_bytes(ikm: &[u8]) -> Self {
        let sk = blst_core::SecretKey::key_gen(&ikm, &[]).expect("unable to generate a secret key");
        SecretKey(sk)
    }

    pub fn public_key(&self) -> PublicKey {
        let pk = self.0.sk_to_pk();
        PublicKey(pk)
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        Signature(self.0.sign(msg, dst, &[]))
    }
}

#[derive(Debug)]
pub struct PublicKey(pub(crate) blst_core::PublicKey);

impl PublicKey {
    pub fn verify_signature(&self, sig: Signature, msg: &[u8]) -> bool {
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        let pk = self.0;
        let avg = &[];
        let err = sig.0.verify(true, msg, dst, avg, &pk, true);
        err == BLST_ERROR::BLST_SUCCESS
    }

    pub fn validate(&self) -> bool {
        let valid = self.0.validate();
        match valid {
            Ok(()) => true,
            Err(_) => false
        }
    }
}

#[derive(Debug)]
pub struct Signature(pub(crate) blst_core::Signature);

impl Signature {
    pub fn verify(&self, pk: PublicKey, msg: &[u8]) -> bool {
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        let avg = &[];
        let err = self.0.verify(true, msg, dst, avg, &pk.0, true);
        err == BLST_ERROR::BLST_SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::SecretKey;

    #[test]
    fn signature() {
        let sk = SecretKey::random();
        let pk = sk.public_key();
        let msg = "message";
        let sig = sk.sign(msg.as_ref());

        assert!(sig.verify(pk, msg.as_ref()));

        let pk = sk.public_key();
        assert!(pk.verify_signature(sig, msg.as_ref()));
    }

    #[test]
    #[should_panic(expected = "unable to generate a secret key")]
    fn secret_key_is_null() {
        let ikm = [0u8;0];
        SecretKey::from_bytes(&ikm);
    }

    #[test]
    #[should_panic(expected = "unable to generate a secret key")]
    fn secret_key_len_31() {
        let ikm = [1u8;31];
        SecretKey::from_bytes(&ikm);
    }

    #[test]
    fn valid_public_key() {
        let pk = SecretKey::random().public_key();
        let valid = pk.validate();
        assert!(valid)
    }
}


