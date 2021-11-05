use crate::primitives::Bytes32;
use blst::{min_pk as blst_core, BLST_ERROR};
use rand::prelude::*;
use sha2::{digest::FixedOutput, Digest, Sha256};
use ssz_rs::prelude::*;
use std::fmt;
use std::ops::DerefMut;
use thiserror::Error;

const BLS_DST: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

pub fn hash(data: &[u8]) -> Bytes32 {
    let mut result = Bytes32::default();
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize_into(result.deref_mut().as_mut_slice().into());
    result
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Size mismatch")]
    SizeMismatch,
    #[error("Aggregation error")]
    AggregateError,
    #[error("Zero sized input")]
    ZeroSizedInput,
}

#[derive(Clone)]
pub struct SecretKey(blst_core::SecretKey);

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl fmt::LowerHex for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in self.0.to_bytes() {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl SecretKey {
    pub fn random() -> Self {
        let mut ikm = [0u8; 32];
        let mut rng = rand::thread_rng();
        rng.try_fill_bytes(&mut ikm)
            .expect("unable to generate key material");
        Self::from_bytes(&ikm)
    }

    pub fn from_bytes(ikm: &[u8]) -> Self {
        let sk = blst_core::SecretKey::key_gen(ikm, &[]).expect("unable to generate a secret key");
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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct PublicKey(blst_core::PublicKey);

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in self.0.to_bytes() {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl PublicKey {
    pub fn verify_signature(&self, msg: &[u8], sig: Signature) -> bool {
        let pk = self.0;
        let avg = &[];
        let res = sig.0.verify(true, msg, BLS_DST, avg, &pk, true);
        res == BLST_ERROR::BLST_SUCCESS
    }

    pub fn validate(&self) -> bool {
        self.0.validate().is_ok()
    }
}

impl Sized for PublicKey {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        48
    }
}

impl Serialize for PublicKey {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        let start = buffer.len();
        buffer.extend_from_slice(&self.0.to_bytes());
        let encoded_length = buffer.len() - start;
        debug_assert!(encoded_length == Self::size_hint());
        Ok(encoded_length)
    }
}

impl Deserialize for PublicKey {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let inner = blst_core::PublicKey::deserialize(encoding)
            .map_err(|_| DeserializeError::InvalidInput)?;
        Ok(Self(inner))
    }
}

impl Merkleized for PublicKey {
    fn hash_tree_root(&self, context: &MerkleizationContext) -> Result<Root, MerkleizationError> {
        let mut buffer = vec![];
        self.serialize(&mut buffer)?;
        pack_bytes(&mut buffer);
        merkleize(&buffer, None, context)
    }
}

impl SimpleSerialize for PublicKey {
    fn is_composite_type() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Signature(blst_core::Signature);

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in self.0.to_bytes() {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl Default for Signature {
    fn default() -> Self {
        let mut encoding = vec![0u8; Self::size_hint()];
        // set top two bits, particularity of the `BLS12-381` encoding
        encoding[0] = 192;
        Self(
            blst_core::Signature::from_bytes(&encoding)
                .expect("default is a well-formed signature"),
        )
    }
}

impl Signature {
    pub fn verify(&self, pk: PublicKey, msg: &[u8]) -> bool {
        let avg = &[];
        let err = self.0.verify(true, msg, BLS_DST, avg, &pk.0, true);
        err == BLST_ERROR::BLST_SUCCESS
    }

    pub fn from_bytes(b: &[u8]) -> Self {
        let sig =
            blst_core::Signature::from_bytes(b).expect("unable to create a signature from bytes");
        Signature(sig)
    }
}

impl Sized for Signature {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        96
    }
}

impl Serialize for Signature {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        let start = buffer.len();
        buffer.extend_from_slice(&self.0.to_bytes());
        let encoded_length = buffer.len() - start;
        debug_assert!(encoded_length == Self::size_hint());
        Ok(encoded_length)
    }
}

impl Deserialize for Signature {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        let inner = blst_core::Signature::deserialize(encoding)
            .map_err(|_| DeserializeError::InvalidInput)?;
        Ok(Self(inner))
    }
}

impl Merkleized for Signature {
    fn hash_tree_root(&self, context: &MerkleizationContext) -> Result<Root, MerkleizationError> {
        let mut buffer = vec![];
        self.serialize(&mut buffer)?;
        pack_bytes(&mut buffer);
        merkleize(&buffer, None, context)
    }
}

impl SimpleSerialize for Signature {
    fn is_composite_type() -> bool {
        false
    }
}

pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }
    let vs: Vec<&blst_core::Signature> = signatures.iter().map(|s| &s.0).collect();

    blst_core::AggregateSignature::aggregate(&vs, true)
        .map(|s| Signature(s.to_signature()))
        .map_err(|_| Error::SizeMismatch)
}

pub fn aggregate_verify(pks: &[PublicKey], msgs: &[&[u8]], signature: Signature) -> bool {
    let v: Vec<&blst_core::PublicKey> = pks.iter().map(|pk| &pk.0).collect();
    let res = signature.0.aggregate_verify(true, msgs, BLS_DST, &v, true);
    res == BLST_ERROR::BLST_SUCCESS
}

pub fn fast_aggregate_verify(pks: &[PublicKey], msg: &[u8], signature: Signature) -> bool {
    let v: Vec<&blst_core::PublicKey> = pks.iter().map(|pk| &pk.0).collect();
    let res = signature.0.fast_aggregate_verify(true, msg, BLS_DST, &v);
    res == BLST_ERROR::BLST_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::{SecretKey, Signature};
    use crate::crypto::{aggregate, aggregate_verify, fast_aggregate_verify};
    use rand::prelude::*;

    #[test]
    fn signature() {
        let sk = SecretKey::random();
        let pk = sk.public_key();
        let msg = "message";
        let sig = sk.sign(msg.as_ref());

        assert!(sig.verify(pk, msg.as_ref()));

        let pk = sk.public_key();
        assert!(pk.verify_signature(msg.as_ref(), sig));
    }

    #[test]
    #[should_panic(expected = "unable to create a signature from bytes")]
    fn test_signature_from_null_bytes() {
        let b = [0u8; 0];
        Signature::from_bytes(&b);
    }

    #[test]
    fn test_signature_from_good_bytes() {
        let b = [
            0xab, 0xb0, 0x12, 0x4c, 0x75, 0x74, 0xf2, 0x81, 0xa2, 0x93, 0xf4, 0x18, 0x5c, 0xad,
            0x3c, 0xb2, 0x26, 0x81, 0xd5, 0x20, 0x91, 0x7c, 0xe4, 0x66, 0x65, 0x24, 0x3e, 0xac,
            0xb0, 0x51, 0x00, 0x0d, 0x8b, 0xac, 0xf7, 0x5e, 0x14, 0x51, 0x87, 0x0c, 0xa6, 0xb3,
            0xb9, 0xe6, 0xc9, 0xd4, 0x1a, 0x7b, 0x02, 0xea, 0xd2, 0x68, 0x5a, 0x84, 0x18, 0x8a,
            0x4f, 0xaf, 0xd3, 0x82, 0x5d, 0xaf, 0x6a, 0x98, 0x96, 0x25, 0xd7, 0x19, 0xcc, 0xd2,
            0xd8, 0x3a, 0x40, 0x10, 0x1f, 0x4a, 0x45, 0x3f, 0xca, 0x62, 0x87, 0x8c, 0x89, 0x0e,
            0xca, 0x62, 0x23, 0x63, 0xf9, 0xdd, 0xb8, 0xf3, 0x67, 0xa9, 0x1e, 0x84,
        ];
        Signature::from_bytes(&b);
    }

    #[test]
    #[should_panic(expected = "unable to generate a secret key")]
    fn secret_key_is_null() {
        let ikm = [0u8; 0];
        SecretKey::from_bytes(&ikm);
    }

    #[test]
    #[should_panic(expected = "unable to generate a secret key")]
    fn secret_key_len_31() {
        let ikm = [1u8; 31];
        SecretKey::from_bytes(&ikm);
    }

    #[test]
    fn valid_public_key() {
        let pk = SecretKey::random().public_key();
        let valid = pk.validate();
        assert!(valid)
    }

    #[test]
    fn test_aggregate_verify() {
        let n = 20;
        let sks: Vec<_> = (0..n).map(|_| SecretKey::random()).collect();
        let pks: Vec<_> = sks.iter().map(|sk| sk.public_key()).collect();
        let msgs: Vec<Vec<u8>> = (0..n)
            .map(|_| (0..64).map(|_| rand::thread_rng().gen()).collect())
            .collect();

        let signatures: Vec<_> = msgs
            .iter()
            .zip(&sks)
            .map(|(msg, sk)| sk.sign(msg.as_ref()))
            .collect();

        let msgs = msgs.iter().map(|r| &r[..]).collect::<Vec<_>>();

        let sig = aggregate(signatures.as_ref()).unwrap();
        let v = aggregate_verify(pks.as_slice(), msgs.as_ref(), sig);

        assert!(v);
    }

    #[test]
    fn test_fast_aggregated_verify() {
        let n = 20;
        let sks: Vec<_> = (0..n).map(|_| SecretKey::random()).collect();
        let pks: Vec<_> = sks.iter().map(|sk| sk.public_key()).collect();
        let msg = "message";

        let signatures: Vec<_> = sks.iter().map(|sk| sk.sign(msg.as_ref())).collect();

        let sig = aggregate(signatures.as_slice()).unwrap();
        let v = fast_aggregate_verify(pks.as_slice(), msg.as_ref(), sig);

        assert!(v);
    }

    #[test]
    fn test_can_make_default_signature() {
        let signature = Signature::default();
        dbg!(signature);
    }
}
