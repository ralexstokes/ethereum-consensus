use crate::primitives::{Epoch, Gwei, VersionBytes};
use ssz_rs::Vector;

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 16384;
pub const MIN_GENESIS_TIME: u64 = 1606824000;
pub const GENESIS_FORK_VERSION_BYTES: VersionBytes = [0u8; 4];
pub const GENESIS_FORK_VERSION: Vector<u8, 4> = Vector::from_iter(GENESIS_FORK_VERSION_BYTES);
pub const GENESIS_DELAY: u64 = 604800;
pub const SECONDS_PER_SLOT: u64 = 12;
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14;
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: Epoch = 256;
pub const SHARD_COMMITTEE_PERIOD: Epoch = 256;
pub const ETH1_FOLLOW_DISTANCE: u64 = 2048;
pub const EJECTION_BALANCE: Gwei = 16 * 10u64.pow(9);
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
pub const CHURN_LIMIT_QUOTIENT: u64 = 65536;
