use heck::ToSnakeCase;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Config {
    General,
    Minimal,
    Mainnet,
}

impl From<&str> for Config {
    fn from(value: &str) -> Self {
        match value {
            "general" => Self::General,
            "minimal" => Self::Minimal,
            "mainnet" => Self::Mainnet,
            _ => panic!("unsupported config"),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = format!("{self:?}");
        write!(f, "{0}", inner.to_snake_case())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Fork {
    Phase0,
    Altair,
    Bellatrix,
    Capella,
    Deneb,
    Electra,
    Eip6110,
    Whisk,
}

impl From<&str> for Fork {
    fn from(value: &str) -> Self {
        match value {
            "phase0" => Self::Phase0,
            "altair" => Self::Altair,
            "bellatrix" => Self::Bellatrix,
            "capella" => Self::Capella,
            "deneb" => Self::Deneb,
            "electra" => Self::Electra,
            "eip6110" => Self::Eip6110,
            "whisk" => Self::Whisk,
            fork => panic!("unsupported fork: {fork:?}"),
        }
    }
}

impl fmt::Display for Fork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = format!("{self:?}");
        write!(f, "{0}", inner.to_snake_case())
    }
}

#[derive(Debug)]
pub enum Runner {
    Bls,
    EpochProcessing,
    Finality,
    ForkChoice,
    Fork,
    Genesis,
    Kzg,
    LightClient,
    MerkleProof,
    Operations,
    Random,
    Rewards,
    Sanity,
    Shuffling,
    SszStatic,
    SszGeneric,
    Sync,
    Transition,
}

impl Runner {
    pub fn should_ignore(&self) -> bool {
        matches!(self, Self::ForkChoice | Self::Sync)
    }

    // Do not collect these tests.
    pub fn should_skip(&self) -> bool {
        matches!(self, Self::SszGeneric)
    }
}

impl From<&str> for Runner {
    fn from(value: &str) -> Self {
        match value {
            "bls" => Self::Bls,
            "epoch_processing" => Self::EpochProcessing,
            "finality" => Self::Finality,
            "fork_choice" => Self::ForkChoice,
            "fork" => Self::Fork,
            "genesis" => Self::Genesis,
            "kzg" => Self::Kzg,
            "light_client" => Self::LightClient,
            "merkle_proof" => Self::MerkleProof,
            "operations" => Self::Operations,
            "random" => Self::Random,
            "rewards" => Self::Rewards,
            "sanity" => Self::Sanity,
            "shuffling" => Self::Shuffling,
            "ssz_static" => Self::SszStatic,
            "ssz_generic" => Self::SszGeneric,
            "sync" => Self::Sync,
            "transition" => Self::Transition,
            runner => panic!("unsupported runner: {runner}"),
        }
    }
}

impl fmt::Display for Runner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = format!("{self:?}");
        write!(f, "{0}", inner.to_snake_case())
    }
}

#[derive(Debug)]
pub struct Handler(pub String);

impl From<&str> for Handler {
    fn from(value: &str) -> Self {
        let inner = if value.contains("BLSToExecutionChange") {
            value.replace("BLS", "Bls")
        } else {
            value.to_string()
        };
        Self(inner)
    }
}

impl fmt::Display for Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.0.to_snake_case())
    }
}

#[derive(Debug)]
pub struct Suite(pub String);

impl From<&str> for Suite {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.0.to_snake_case())
    }
}

#[derive(Debug)]
pub struct Case(pub String);

impl From<&str> for Case {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.0.to_snake_case())
    }
}

pub struct TestMeta {
    pub config: Config,
    pub fork: Fork,
    pub runner: Runner,
    pub handler: Handler,
    pub suite: Suite,
    pub case: Case,
}

impl fmt::Display for TestMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}::", &self.config)?;
        write!(f, "{0}::", &self.fork)?;
        write!(f, "{0}::", &self.runner)?;
        write!(f, "{0}::", &self.handler)?;
        write!(f, "{0}::", &self.suite)?;
        write!(f, "{0}", &self.case)
    }
}

impl TestMeta {
    pub fn name(&self) -> String {
        format!("{self}")
    }

    // Mark this test as "ignored", which does collect it but does not try to run the test.
    // If ignored, a test could be implemented in the future but is currently not.
    pub fn should_ignore(&self) -> bool {
        let ignored_runner = self.runner.should_ignore();
        let ignored_handler =
            matches!(self.runner, Runner::LightClient) && self.handler.0 != "single_merkle_proof";
        ignored_runner || ignored_handler
    }

    // Skip collecting this test if `true`.
    // If not collected, a test is completely ignored from the spec testing
    // as it is not currently supported and there is no intention to support in this repo.
    pub fn should_skip(&self) -> bool {
        let skipped_runner = self.runner.should_skip();
        let skipped_forks = matches!(self.fork, Fork::Eip6110 | Fork::Whisk);
        skipped_runner | skipped_forks
    }
}
