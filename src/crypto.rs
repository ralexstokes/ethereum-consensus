use ssz_rs::prelude::*;
use rand::prelude::*;

use blst::min_pk as blst;

pub const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;
pub const BLS_SECRET_KEY_BYTES_LEN: usize = 32;
pub const BLS_SIGNATURE_BYTES_LEN: usize = 96;

pub type BLSPubkey = Vector<u8, BLS_PUBLIC_KEY_BYTES_LEN>;
pub type BLSSignature = Vector<u8, BLS_SIGNATURE_BYTES_LEN>;

pub struct Signature(pub(crate) blst::Signature);

#[derive(Debug)]
pub struct SecretKey(pub(crate) blst::SecretKey);

impl SecretKey {
    pub fn random() -> Self {
        let mut ikm=[0u8; 32];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut ikm);
        let sk = blst::SecretKey::key_gen(&ikm, &[]).expect("unable to generate a secret key");
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
pub struct PublicKey(pub(crate) blst::PublicKey);

impl PublicKey {

}


