/// This module contains support for various Ethereum netowrks.
use crate::state_transition::{Context, Error};

/// `Network` describes one of the established networks this repository supports
/// or otherwise a `Custom` variant that wraps a path to a local configuration file
/// for the custom network (useful for devnets).
#[derive(Default, Debug, Clone)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    #[default]
    Mainnet,
    Sepolia,
    Goerli,
    Holesky,
    Custom(String),
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Sepolia => write!(f, "sepolia"),
            Self::Goerli => write!(f, "goerli"),
            Self::Holesky => write!(f, "holesky"),
            Self::Custom(config_file) => write!(f, "custom network with config at `{config_file}`"),
        }
    }
}

impl TryFrom<&Network> for Context {
    type Error = Error;

    fn try_from(network: &Network) -> Result<Self, Self::Error> {
        match network {
            Network::Mainnet => Ok(Context::for_mainnet()),
            Network::Sepolia => Ok(Context::for_sepolia()),
            Network::Goerli => Ok(Context::for_goerli()),
            Network::Holesky => Ok(Context::for_holesky()),
            Network::Custom(config) => Context::try_from_file(config),
        }
    }
}

// NOTE: the default genesis time here is usually seen on testnets
// where we have control over the genesis details
pub fn typical_genesis_time(context: &Context) -> u64 {
    context.min_genesis_time + context.genesis_delay
}
