use thiserror::Error;
use ssz_rs::prelude::*;
use rand::prelude::*;
use blst::{min_pk as blst_core, BLST_ERROR};

pub const BLS_SIGNATURE_BYTES_LEN: usize = 96;
pub const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;
pub const BLS_SECRET_KEY_BYTES_LEN: usize = 32;

const BLS_DST: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

pub type BLSPubkey = Vector<u8, BLS_PUBLIC_KEY_BYTES_LEN>;
pub type BLSSignature = Vector<u8, BLS_SIGNATURE_BYTES_LEN>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("Aggregation error")]
    AggregateError,
    #[error("Zero sized input")]
    ZeroSizedInput,
}

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
        Signature(self.0.sign(msg, BLS_DST, &[]))
    }
}

#[derive(Debug)]
pub struct PublicKey(pub(crate) blst_core::PublicKey);

impl PublicKey {
    pub fn verify_signature(&self, sig: Signature, msg: &[u8]) -> bool {
        let pk = self.0;
        let avg = &[];
        let res = sig.0.verify(true, msg, BLS_DST, avg, &pk, true);
        res == BLST_ERROR::BLST_SUCCESS
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
        let avg = &[];
        let err = self.0.verify(true, msg, BLS_DST, avg, &pk.0, true);
        err == BLST_ERROR::BLST_SUCCESS
    }
}

pub fn aggregate(signatures: &[Signature])-> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }
    let vs: Vec<&blst_core::Signature> = signatures.into_iter().map(|s| &s.0).collect();

    return blst_core::AggregateSignature::aggregate(&vs, true)
        .map(|s| Signature(s.to_signature()))
        .map_err(|_| Error::SizeMismatch)
}

pub fn aggregate_verify(signature: Signature, msgs: &[&[u8]], pks: &[PublicKey]) -> bool {
    let v: Vec<&blst_core::PublicKey> = pks.into_iter().map(|pk| &pk.0).collect();
    let res = signature.0.aggregate_verify(true, msgs, BLS_DST, &v, true);
    res == BLST_ERROR::BLST_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::SecretKey;
    use crate::crypto::{aggregate, aggregate_verify};

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

    #[test]
    fn aggregated_signatures() {
        let sk1 = SecretKey::random();
        let pk1 = sk1.public_key();

        let sk2 = SecretKey::random();
        let pk2 = sk2.public_key();

        let msg1 = "message1";
        let msg2 = "message2";

        let sig1 = sk1.sign(msg1.as_ref());
        let sig2 = sk2.sign(msg2.as_ref());

        let agg = aggregate(&[sig1, sig2]).unwrap();
        let v = aggregate_verify(agg, &[msg1.as_ref(), msg2.as_ref()], &[pk1, pk2]);

        assert!(v);
    }
}


