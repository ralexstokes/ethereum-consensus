//! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
pub mod block_processing;
pub mod epoch_processing;
pub mod genesis;
pub mod helpers;
pub mod slot_processing;
use crate::bellatrix as spec;
use crate::state_transition::{Context, Error, Result, Validation};
use spec::{
    process_block, process_slots, verify_block_signature, ParticipationFlags,
    SyncAggregate,
};
use ssz_rs::prelude::*;
