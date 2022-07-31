use crate::bytes::write_bytes_to_lower_hex;
#[cfg(feature = "serde")]
use crate::serde::{try_bytes_from_hex_str, HexError};
use crate::ssz::ByteVector;
#[cfg(feature = "serde")]
use serde;
use ssz_rs::prelude::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use thiserror::Error;

const BLS_PUBLIC_KEY_BYTES_LEN: usize = 48;
const BLS_SECRET_KEY_BYTES_LEN: usize = 32;
const BLS_SIGNATURE_BYTES_LEN: usize = 96;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "serde")]
    #[error("error deserializing: {0}")]
    Deserialize(#[from] HexError),
    #[error("inputs required but none were provided")]
    EmptyInput,
    #[error("randomness failure: {0}")]
    Randomness(#[from] rand::Error),
    #[error("expected additional input data when decoding")]
    MissingInput,
}

pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::EmptyInput);
    }
    Ok(signatures[0].clone())
}

pub fn aggregate_verify(_pks: &[PublicKey], _msgs: &[&[u8]], _signature: &Signature) -> bool {
    true
}

pub fn fast_aggregate_verify(_pks: &[&PublicKey], _msg: &[u8], _signature: &Signature) -> bool {
    true
}

// Return the aggregate public key for the public keys in `pks`
pub fn eth_aggregate_public_keys(pks: &[PublicKey]) -> Result<PublicKey, Error> {
    if pks.is_empty() {
        return Err(Error::EmptyInput);
    }

    Ok(pks[0].clone())
}

pub fn eth_fast_aggregate_verify(
    public_keys: &[&PublicKey],
    message: &[u8],
    signature: &Signature,
) -> bool {
    if public_keys.is_empty() && signature.is_infinity() {
        true
    } else {
        fast_aggregate_verify(public_keys, message, signature)
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
pub struct SecretKey(ByteVector<BLS_SECRET_KEY_BYTES_LEN>);

impl fmt::LowerHex for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self.as_bytes())
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretKey({:#x})", self)
    }
}

#[cfg(feature = "serde")]
impl TryFrom<String> for SecretKey {
    type Error = Error;

    fn try_from(data: String) -> Result<Self, Self::Error> {
        let encoding = try_bytes_from_hex_str(&data)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != BLS_SECRET_KEY_BYTES_LEN {
            return Err(Error::MissingInput);
        }
        let inner = ByteVector::<BLS_SECRET_KEY_BYTES_LEN>::try_from(data).unwrap();
        Ok(Self(inner))
    }
}

impl PartialEq for SecretKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for SecretKey {}

impl SecretKey {
    // https://docs.rs/rand/latest/rand/trait.Rng.html#generic-usage
    pub fn random<R: rand::Rng>(rng: &mut R) -> Result<Self, Error> {
        let mut ikm = [0u8; BLS_SECRET_KEY_BYTES_LEN];
        rng.try_fill_bytes(&mut ikm)?;
        Self::key_gen(&ikm)
    }

    pub fn key_gen(ikm: &[u8]) -> Result<Self, Error> {
        Self::try_from(ikm)
    }

    pub fn as_bytes(&self) -> [u8; BLS_SECRET_KEY_BYTES_LEN] {
        self.0.to_bytes()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(Default::default())
    }

    pub fn sign(&self, _msg: &[u8]) -> Signature {
        Signature::default()
    }
}

#[derive(Default, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct PublicKey(ByteVector<BLS_PUBLIC_KEY_BYTES_LEN>);

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self.as_bytes())
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({:#x})", self)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

#[cfg(feature = "serde")]
impl From<PublicKey> for String {
    fn from(key: PublicKey) -> Self {
        format!("{key}")
    }
}

#[cfg(feature = "serde")]
impl TryFrom<String> for PublicKey {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Error> {
        let encoding = try_bytes_from_hex_str(&s)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        if data.len() != BLS_PUBLIC_KEY_BYTES_LEN {
            return Err(Error::MissingInput);
        }
        let inner = ByteVector::<BLS_PUBLIC_KEY_BYTES_LEN>::try_from(data).unwrap();
        Ok(Self(inner))
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PublicKey {
    pub fn verify_signature(&self, msg: &[u8], sig: &Signature) -> bool {
        sig.verify(self, msg)
    }

    pub fn validate(&self) -> bool {
        true
    }

    pub fn as_bytes(&self) -> [u8; BLS_PUBLIC_KEY_BYTES_LEN] {
        self.0.to_bytes()
    }
}

impl Sized for PublicKey {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        BLS_PUBLIC_KEY_BYTES_LEN
    }
}

impl Serialize for PublicKey {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        let start = buffer.len();
        buffer.extend_from_slice(&self.as_bytes());
        let encoded_length = buffer.len() - start;
        debug_assert_eq!(encoded_length, Self::size_hint());
        Ok(encoded_length)
    }
}

impl Deserialize for PublicKey {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized,
    {
        Self::try_from(encoding).map_err(|_| DeserializeError::InvalidInput)
    }
}

impl Merkleized for PublicKey {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        let mut buffer = vec![];
        self.serialize(&mut buffer)?;
        pack_bytes(&mut buffer);
        merkleize(&buffer, None)
    }
}

impl SimpleSerialize for PublicKey {
    fn is_composite_type() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct Signature(ByteVector<BLS_SIGNATURE_BYTES_LEN>);

impl fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_bytes_to_lower_hex(f, self.as_bytes())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({:#x})", self)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

#[cfg(feature = "serde")]
impl From<Signature> for String {
    fn from(signature: Signature) -> Self {
        format!("{signature}")
    }
}

#[cfg(feature = "serde")]
impl TryFrom<String> for Signature {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Error> {
        let encoding = try_bytes_from_hex_str(&s)?;
        Self::try_from(encoding.as_ref())
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        if data.len() != BLS_SIGNATURE_BYTES_LEN {
            return Err(Error::MissingInput);
        }
        let inner = ByteVector::<BLS_SIGNATURE_BYTES_LEN>::try_from(data).unwrap();
        Ok(Self(inner))
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Signature {
    pub fn verify(&self, _pk: &PublicKey, _msg: &[u8]) -> bool {
        true
    }

    pub fn as_bytes(&self) -> [u8; 96] {
        self.0.to_bytes()
    }

    pub fn is_infinity(&self) -> bool {
        self == &Self::default()
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
        buffer.extend_from_slice(&self.as_bytes());
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
        let signature = Self::try_from(encoding).map_err(|_| DeserializeError::InvalidInput)?;
        Ok(signature)
    }
}

impl Merkleized for Signature {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        let mut buffer = vec![];
        self.serialize(&mut buffer)?;
        pack_bytes(&mut buffer);
        merkleize(&buffer, None)
    }
}

impl SimpleSerialize for Signature {
    fn is_composite_type() -> bool {
        false
    }
}
