use ethereum_types::H256;
use ssz_rs::prelude::*;

pub use ssz_rs::prelude::Root;

pub type Slot = u64;
pub type Epoch = u64;
pub type CommitteeIndex = u64;
pub type ValidatorIndex = u64;
pub type Gwei = u64;
pub type Hash32 = H256;
pub type Bytes32 = Vector<u8, 32>;
pub type Version = Vector<u8, 4>;
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
