pub use crate::crypto::{PublicKey as BlsPublicKey, Signature as BlsSignature};
use ssz_rs::prelude::*;
use std::convert::AsRef;
use std::fmt;

pub type Root = Node;
pub type Slot = u64;
pub type Epoch = u64;
pub type CommitteeIndex = usize;
pub type ValidatorIndex = usize;
pub type Gwei = u64;
pub type Hash32 = U256;

pub type Version = [u8; 4];
pub type DomainType = [u8; 4];
pub type ForkDigest = [u8; 4];
pub type Domain = [u8; 32];

pub type ExecutionAddress = Vector<u8, 20>;

pub type ChainId = usize;
pub type NetworkId = usize;

pub type RandaoReveal = BlsSignature;

// Coordinate refers to a unique location in the block tree
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
pub struct Bytes32(pub(crate) Vector<u8, 32>);

impl fmt::LowerHex for Bytes32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Bytes32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl Bytes32 {
    pub fn xor(&self, other: Self) -> Self {
        let mut result = Vector::default();
        for (i, (a, b)) in self.0.iter().zip(other.0.iter()).enumerate() {
            result[i] = a ^ b;
        }
        Self(result)
    }
}

impl AsRef<[u8]> for Bytes32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
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
