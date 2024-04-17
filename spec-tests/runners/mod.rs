pub mod bls;
pub mod epoch_processing;
pub mod finality;
pub mod fork;
pub mod genesis;
pub mod kzg;
pub mod light_client;
pub mod merkle_proof;
pub mod operations;
pub mod random;
pub mod rewards;
pub mod sanity;
pub mod shuffling;
pub mod ssz_static;
pub mod transition;
pub mod utils;

macro_rules! gen_exec {
    ($test_case:expr, $loader_fn:expr, $exec_fn:expr) => {
        let inputs = $loader_fn(&$test_case.data_path);
        let context = $test_case.context();
        $exec_fn(inputs, &context)
    };
}

macro_rules! gen_match_for {
    ($test_case:expr, $(($target_config:ident, $target_fork:ident)),+ $target_body:block) => {
        match ($test_case.meta.config, $test_case.meta.fork) {
            $(
                paste::paste! { (crate::test_meta::Config::[<$target_config:camel>], crate::test_meta::Fork::[<$target_fork:camel>]) } => {
                    paste::paste! { use ethereum_consensus::[<$target_fork>]::[<$target_config>] as spec; }
                    $target_body
                }
            )+
            pair => unreachable!("no tests for {pair:?}"),
        }
    };
    ($test_case:expr, $(($target_config:ident, $target_fork:ident) => $target_body:block)+) => {
        match ($test_case.meta.config, $test_case.meta.fork) {
            $(
                paste::paste! { (crate::test_meta::Config::[<$target_config:camel>], crate::test_meta::Fork::[<$target_fork:camel>]) } => {
                    paste::paste! { use ethereum_consensus::[<$target_fork>]::[<$target_config>] as spec; }
                    $target_body
                }
            )+
            pair => unreachable!("no tests for {pair:?}"),
        }
    };
}

macro_rules! gen_match_for_all {
    ($test_case:expr, $loader_fn:expr, $exec_fn:expr) => {
        crate::runners::gen_match_for! {
            $test_case,
            (mainnet, phase0),
            (mainnet, altair),
            (mainnet, bellatrix),
            (mainnet, capella),
            (mainnet, deneb),
            (minimal, phase0),
            (minimal, altair),
            (minimal, bellatrix),
            (minimal, capella),
            (minimal, deneb)
            {
                gen_exec! { $test_case, $loader_fn, $exec_fn }
            }
        }
    };
}

pub(crate) use gen_exec;
pub(crate) use gen_match_for;
pub(crate) use gen_match_for_all;
