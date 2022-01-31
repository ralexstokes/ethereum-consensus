pub mod beacon_block;
pub mod beacon_state;
pub mod configs;
pub mod light_client;
pub mod presets;
pub mod state_transition;
pub mod sync_aggregate;
pub mod sync_committee;

use ssz_rs::prelude::*;

pub const TIMELY_SOURCE_FLAG_INDEX: u8 = 0;
pub const TIMELY_TARGET_FLAG_INDEX: u8 = 1;
pub const TIMELY_HEAD_FLAG_INDEX: u8 = 2;

pub const TIMELY_SOURCE_WEIGHT: u64 = 14;
pub const TIMELY_TARGET_WEIGHT: u64 = 26;
pub const TIMELY_HEAD_WEIGHT: u64 = 14;
pub const SYNC_REWARD_WEIGHT: u64 = 2;
pub const PROPOSER_WEIGHT: u64 = 8;
pub const WEIGHT_DENOMINATOR: u64 = 64;

pub const PARTICIPATION_FLAG_WEIGHTS: [u64; 3] = [
    TIMELY_SOURCE_WEIGHT,
    TIMELY_TARGET_WEIGHT,
    TIMELY_HEAD_WEIGHT,
];
