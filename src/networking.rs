use crate::primitives::Epoch;
use crate::ssz::ByteList;
use enr;
use ssz_rs::prelude::Bitvector;
use std::fmt;
use std::time::Duration;

pub const ATTESTATION_SUBNET_COUNT: usize = 64;
pub const GOSSIP_MAX_SIZE: usize = 2usize.pow(20);
pub const MAX_REQUEST_BLOCKS: usize = 2usize.pow(10);
pub const MIN_EPOCHS_FOR_BLOCK_REQUESTS: Epoch = 33024;
pub const MAX_CHUNK_SIZE: usize = 2usize.pow(20);
pub const TTFB_TIMEOUT: Duration = Duration::from_secs(5);
pub const RESP_TIMEOUT: Duration = Duration::from_secs(10);
pub const ATTESTATION_PROPAGATION_SLOT_RANGE: usize = 32;
pub const MAXIMUM_GOSSIP_CLOCK_DISPARITY: Duration = Duration::from_millis(500);

pub use multiaddr::Multiaddr;

// TODO see how much this can be tightened
const MAX_PEER_ID_BYTE_COUNT: usize = 512;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PeerId(#[serde(with = "crate::serde::as_b58")] ByteList<MAX_PEER_ID_BYTE_COUNT>);
#[cfg(feature = "serde")]
impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0))
    }
}

pub type Enr = enr::Enr<enr::k256::ecdsa::SigningKey>;

pub enum MessageDomain {
    InvalidSnappy,
    ValidSnappy,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetaData {
    #[serde(with = "crate::serde::as_string")]
    pub seq_number: u64,
    pub attnets: Bitvector<ATTESTATION_SUBNET_COUNT>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_id_serde() {
        let id_repr = "\"16Uiu2HAmVDji3ShrqL9DLnQo3teJcEWiKqy9qKefFFFxrz2EYwde\"";
        let id: PeerId = serde_json::from_str(id_repr).unwrap();
        let roundtrip_id_repr = serde_json::to_string(&id).unwrap();
        assert_eq!(id_repr, roundtrip_id_repr);
    }
}
