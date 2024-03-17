use crate::{
    runners::{
        bls, epoch_processing, finality, fork, genesis, operations, random, rewards, sanity,
        shuffling, ssz_static, transition,
    },
    test_meta::TestMeta,
    Runner::*,
};
use std::{error::Error, path::Path};

pub struct TestCase {
    pub meta: TestMeta,
    pub data_path: String,
}

impl TestCase {
    pub fn new(meta: TestMeta, data_path: &Path) -> Self {
        Self { meta, data_path: data_path.as_os_str().to_str().unwrap().into() }
    }

    pub fn name(&self) -> String {
        self.meta.name()
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
            Kzg => todo!(),
            LightClient => todo!(),
            MerkleProof => todo!(),
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
