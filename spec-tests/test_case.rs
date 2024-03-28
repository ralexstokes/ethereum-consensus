use crate::{
    runners::{
        bls, epoch_processing, finality, fork, genesis, kzg, light_client, merkle_proof,
        operations, random, rewards, sanity, shuffling, ssz_static, transition,
    },
    test_meta::TestMeta,
    Config, Context,
    Runner::*,
};
use ethereum_consensus::state_transition;
use std::{error::Error, path::Path, sync::Arc};

pub struct TestCase {
    pub meta: TestMeta,
    pub data_path: String,
    pub context: Arc<Context>,
}

impl TestCase {
    pub fn new(meta: TestMeta, data_path: &Path, context: Arc<Context>) -> Self {
        Self { meta, data_path: data_path.as_os_str().to_str().unwrap().into(), context }
    }

    pub fn name(&self) -> String {
        self.meta.name()
    }

    pub fn context(&self) -> &state_transition::Context {
        match self.meta.config {
            Config::Mainnet => &self.context.mainnet,
            Config::Minimal => &self.context.minimal,
            _ => unreachable!(),
        }
    }

    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        let result = match &self.meta.runner {
            Bls => bls::dispatch(self),
            EpochProcessing => epoch_processing::dispatch(self),
            Finality => finality::dispatch(self),
            ForkChoice => todo!(),
            Fork => fork::dispatch(self),
            Genesis => genesis::dispatch(self),
            Operations => operations::dispatch(self),
            Random => random::dispatch(self),
            Rewards => rewards::dispatch(self),
            Sanity => sanity::dispatch(self),
            Shuffling => shuffling::dispatch(self),
            SszStatic => ssz_static::dispatch(self),
            Transition => transition::dispatch(self),
            LightClient => light_client::dispatch(self),
            Kzg => kzg::dispatch(self),
            MerkleProof => merkle_proof::dispatch(self),
            Sync => todo!(),
            SszGeneric => unreachable!(),
        };
        match result {
            Err(crate::test_utils::Error::InternalContinue) => {
                panic!("invariant violated; this error should not be surfaced to user")
            }
            other => other.map_err(Into::into),
        }
    }
}
