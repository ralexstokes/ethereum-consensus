pub use crate::crypto::{PublicKey as BlsPublicKey, Signature as BlsSignature};
use crate::ssz::ByteVector;
use ssz_rs::prelude::*;
use std::convert::AsRef;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub type Root = Node;
pub type Slot = u64;
pub type Epoch = u64;

pub type CommitteeIndex = usize;
pub type ValidatorIndex = usize;
pub type Gwei = u64;
pub type Hash32 = Bytes32;

pub type Version = [u8; 4];
pub type DomainType = [u8; 4];
pub type ForkDigest = [u8; 4];
pub type Domain = [u8; 32];

pub type ExecutionAddress = ByteVector<20>;

pub type ChainId = usize;
pub type NetworkId = usize;

pub type RandaoReveal = BlsSignature;

// Coordinate refers to a unique location in the block tree
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Coordinate {
    slot: Slot,
    root: Root,
}

pub const GENESIS_SLOT: Slot = 0;
pub const GENESIS_EPOCH: Epoch = 0;
pub const FAR_FUTURE_EPOCH: Epoch = Epoch::MAX;

pub const BLS_WITHDRAWAL_PREFIX: u8 = 0x00;
pub const ETH1_ADDRESS_WITHDRAWAL_PREFIX: u8 = 0x01;

#[derive(Default, Clone, PartialEq, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bytes32(ByteVector<32>);

impl Bytes32 {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
        Ok(Self(ByteVector::<32>::try_from_bytes(bytes)?))
    }

    pub fn xor(&self, other: Self) -> Self {
        let mut result = Self::default();
        for (i, (a, b)) in self.iter().zip(other.iter()).enumerate() {
            result[i] = a ^ b;
        }
        result
    }
}

impl fmt::Debug for Bytes32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Deref for Bytes32 {
    type Target = ByteVector<32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq<Root> for Bytes32 {
    fn eq(&self, other: &Root) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialEq<Bytes32> for Root {
    fn eq(&self, other: &Bytes32) -> bool {
        self.as_ref() == other.as_ref()
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn test_serde() {
        let bytes = Bytes32::default();
        let json = serde_json::to_string(&bytes).unwrap();
        assert_eq!(
            json,
            "\"0x0000000000000000000000000000000000000000000000000000000000000000\""
        );
        let bytes_roundtrip: Bytes32 = serde_json::from_str(&json).unwrap();
        assert_eq!(bytes, bytes_roundtrip);
    }
}
