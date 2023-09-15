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
            altair::mainnet as altair, bellatrix::mainnet as bellatrix, deneb::mainnet as deneb,
            phase0::mainnet as phase0,
        };

        pub type Client = crate::Client<
            altair::SignedContributionAndProof,
            altair::SyncCommitteeContribution,
            bellatrix::BlindedBeaconBlock,
            bellatrix::SignedBlindedBeaconBlock,
            phase0::Attestation,
            phase0::AttesterSlashing,
            phase0::BeaconBlock,
            phase0::BeaconState,
            phase0::SignedAggregateAndProof,
            phase0::SignedBeaconBlock,
            deneb::BlobSidecar,
            altair::LightClientBootstrap,
            altair::LightClientUpdate,
            altair::LightClientFinalityUpdate,
            altair::LightClientOptimisticUpdate,
        >;
    }
    pub mod minimal {
        use ethereum_consensus::{
            altair::minimal as altair, bellatrix::minimal as bellatrix, deneb::minimal as deneb,
            phase0::minimal as phase0,
        };

        pub type Client = crate::Client<
            altair::SignedContributionAndProof,
            altair::SyncCommitteeContribution,
            bellatrix::BlindedBeaconBlock,
            bellatrix::SignedBlindedBeaconBlock,
            phase0::Attestation,
            phase0::AttesterSlashing,
            phase0::BeaconBlock,
            phase0::BeaconState,
            phase0::SignedAggregateAndProof,
            phase0::SignedBeaconBlock,
            deneb::BlobSidecar,
            altair::LightClientBootstrap,
            altair::LightClientUpdate,
            altair::LightClientFinalityUpdate,
            altair::LightClientOptimisticUpdate,
        >;
    }
}
