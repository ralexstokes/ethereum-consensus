use std::path::PathBuf;

/// This module contains support for various Ethereum netowrks.
use crate::state_transition::Context;
use crate::Error;

/// `Network` describes one of the established networks this repository supports
/// or otherwise a `Custom` variant that wraps a path to a local configuration directory
/// for the custom network (useful for devnets).
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase", into = "String", from = "String")]
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
            Self::Custom(config_dir) => write!(f, "{config_dir}"),
        }
    }
}

impl From<Network> for String {
    fn from(value: Network) -> Self {
        format!("{value}")
    }
}

impl From<String> for Network {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mainnet" => Self::Mainnet,
            "sepolia" => Self::Sepolia,
            "goerli" => Self::Goerli,
            "holesky" => Self::Holesky,
            _ => Self::Custom(value),
        }
    }
}

impl TryFrom<Network> for Context {
    type Error = Error;

    fn try_from(network: Network) -> Result<Self, Self::Error> {
        match network {
            Network::Mainnet => Ok(Context::for_mainnet()),
            Network::Sepolia => Ok(Context::for_sepolia()),
            Network::Goerli => Ok(Context::for_goerli()),
            Network::Holesky => Ok(Context::for_holesky()),
            Network::Custom(config) => {
                let config_file = PathBuf::from(config).join("config.yaml");
                Context::try_from_file(config_file)
            }
        }
    }
}

// NOTE: the default genesis time here is usually seen on testnets
// where we have control over the genesis details
pub fn typical_genesis_time(context: &Context) -> u64 {
    context.min_genesis_time + context.genesis_delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    struct File {
        network: Network,
    }

    #[test]
    fn test_serde() {
        let file = File { network: Network::Custom("/path/to/foo.yaml".to_string()) };
        let str = toml::to_string(&file).unwrap();
        let recovered_file: File = toml::from_str(&str).unwrap();
        assert_eq!(file, recovered_file);
    }
}
