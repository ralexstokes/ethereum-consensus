pub mod bls;
pub mod epoch_processing;
pub mod finality;
pub mod fork;
pub mod genesis;
pub mod operations;
pub mod random;
pub mod rewards;
pub mod sanity;
pub mod shuffling;
pub mod ssz_static;
pub mod transition;

macro_rules! gen_exec {
    ($path:expr, $config:expr, $loader_fn:expr, $exec_fn:expr) => {
        use crate::test_meta::Config;
        use ethereum_consensus::state_transition::Context;

        let context = match $config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
            _ => unreachable!(),
        };
        let inputs = $loader_fn($path);
        $exec_fn(inputs, &context)
    };
    ($spec_import:item $path:expr, $config:expr, $loader_fn:expr, $exec_fn:expr) => {
        $spec_import
        use crate::test_meta::Config;
        use ethereum_consensus::state_transition::Context;

        let context = match $config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
            _ => unreachable!(),
        };
        let inputs = $loader_fn($path);
        $exec_fn(inputs, &context)
    };
}

macro_rules! gen_dispatch {
    ($path:expr, $config:expr, $fork:expr, $loader_fn:expr, $exec_fn:expr) => {
        use crate::test_meta::{Config, Fork};
        match $config {
            Config::Mainnet => match $fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::mainnet as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::mainnet as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::mainnet as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::mainnet as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::mainnet as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
            },
            Config::Minimal => match $fork {
                Fork::Phase0 => {
                    gen_exec! {
                        use ethereum_consensus::phase0::minimal as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Altair => {
                    gen_exec! {
                        use ethereum_consensus::altair::minimal as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Bellatrix => {
                    gen_exec! {
                        use ethereum_consensus::bellatrix::minimal as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Capella => {
                    gen_exec! {
                        use ethereum_consensus::capella::minimal as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
                Fork::Deneb => {
                    gen_exec! {
                        use ethereum_consensus::deneb::minimal as spec;
                        $path, $config, $loader_fn, $exec_fn
                    }
                }
            },
            _ => unreachable!(),
        }
    };
}

pub(crate) use gen_dispatch;
pub(crate) use gen_exec;
