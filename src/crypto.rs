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
        rng.try_fill_bytes(&mut ikm).expect("unable to generate random data");
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
    pub fn verify(&self, sig: Signature, msg: &[u8]) -> bool {
        let dst = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
        let pk = self.0;
        let avg = &[];
        let err = sig.0.verify(true, msg, dst, avg, &pk, true);
        err == BLST_ERROR::BLST_SUCCESS
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
        assert!(pk.verify(sig, msg.as_ref()));
    }


}


