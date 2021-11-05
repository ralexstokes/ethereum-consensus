use ssz_rs::prelude::*;
use std::fmt;

pub type Root = Node;
pub type Slot = u64;
pub type Epoch = u64;
pub type CommitteeIndex = usize;
pub type ValidatorIndex = usize;
pub type Gwei = u64;
pub type Hash32 = U256;

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

pub type Version = Vector<u8, 4>;
pub(crate) type VersionBytes = [u8; 4];
pub type DomainType = Vector<u8, 4>;
pub type ForkDigest = Vector<u8, 4>;
pub type Domain = Vector<u8, 32>;

pub const GENESIS_SLOT: Slot = 0;
pub const GENESIS_EPOCH: Epoch = 0;
pub const FAR_FUTURE_EPOCH: Epoch = Epoch::MAX;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: usize = 2usize.pow(5);
pub const JUSTIFICATION_BITS_LENGTH: usize = 4;

pub const BLS_WITHDRAWAL_PREFIX: u8 = 0x00;
pub const ETH1_ADDRESS_WITHDRAWAL_PREFIX: u8 = 0x01;
