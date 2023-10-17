mod api_client;
mod api_error;
mod cli;
mod serde;
mod types;

pub use api_client::*;
pub use api_error::*;
pub use cli::*;
pub use error::*;
pub use presets::*;
pub use types::*;

mod error {
    use crate::ApiError;
    use thiserror::Error;
    use url::ParseError;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("could not parse URL: {0}")]
        Url(#[from] ParseError),
        #[error("could not send request: {0}")]
        Http(#[from] reqwest::Error),
        #[error("error from API: {0}")]
        Api(#[from] ApiError),
        #[error("missing expected data in response: {0}")]
        MissingExpectedData(String),
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
    }
}

pub mod presets {
    pub mod mainnet {
        use ethereum_consensus::{
            altair::mainnet as altair,
            deneb::mainnet as deneb,
            phase0::mainnet as phase0,
            types::mainnet::{
                BeaconBlock, BeaconState, BlindedBeaconBlock, SignedBeaconBlock,
                SignedBlindedBeaconBlock,
            },
        };

        #[derive(Clone)]
        pub struct MainnetClientTypes;

        impl crate::ClientTypes for MainnetClientTypes {
            type SignedContributionAndProof = altair::SignedContributionAndProof;
            type SyncCommitteeContribution = altair::SyncCommitteeContribution;
            type BlindedBeaconBlock = BlindedBeaconBlock;
            type SignedBlindedBeaconBlock = SignedBlindedBeaconBlock;
            type Attestation = phase0::Attestation;
            type AttesterSlashing = phase0::AttesterSlashing;
            type BeaconBlock = BeaconBlock;
            type BeaconState = BeaconState;
            type SignedAggregateAndProof = phase0::SignedAggregateAndProof;
            type SignedBeaconBlock = SignedBeaconBlock;
            type BlobSidecar = deneb::BlobSidecar;
            type LightClientBootstrap = altair::LightClientBootstrap;
            type LightClientUpdate = altair::LightClientUpdate;
            type LightClientFinalityUpdate = altair::LightClientFinalityUpdate;
            type LightClientOptimisticUpdate = altair::LightClientOptimisticUpdate;
        }

        pub type Client = crate::Client<MainnetClientTypes>;
    }
    pub mod minimal {
        use ethereum_consensus::{
            altair::minimal as altair,
            deneb::minimal as deneb,
            phase0::minimal as phase0,
            types::minimal::{
                BeaconBlock, BeaconState, BlindedBeaconBlock, SignedBeaconBlock,
                SignedBlindedBeaconBlock,
            },
        };

        #[derive(Clone)]
        pub struct MinimalClientTypes;

        impl crate::ClientTypes for MinimalClientTypes {
            type SignedContributionAndProof = altair::SignedContributionAndProof;
            type SyncCommitteeContribution = altair::SyncCommitteeContribution;
            type BlindedBeaconBlock = BlindedBeaconBlock;
            type SignedBlindedBeaconBlock = SignedBlindedBeaconBlock;
            type Attestation = phase0::Attestation;
            type AttesterSlashing = phase0::AttesterSlashing;
            type BeaconBlock = BeaconBlock;
            type BeaconState = BeaconState;
            type SignedAggregateAndProof = phase0::SignedAggregateAndProof;
            type SignedBeaconBlock = SignedBeaconBlock;
            type BlobSidecar = deneb::BlobSidecar;
            type LightClientBootstrap = altair::LightClientBootstrap;
            type LightClientUpdate = altair::LightClientUpdate;
            type LightClientFinalityUpdate = altair::LightClientFinalityUpdate;
            type LightClientOptimisticUpdate = altair::LightClientOptimisticUpdate;
        }

        pub type Client = crate::Client<MinimalClientTypes>;
    }
}
