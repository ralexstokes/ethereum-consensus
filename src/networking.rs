use crate::lib::*;
use crate::primitives::Epoch;
use enr;
pub use multiaddr::Multiaddr;
use multihash::{Code, Error, Multihash};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ssz_rs::prelude::Bitvector;
pub const ATTESTATION_SUBNET_COUNT: usize = 64;
pub const GOSSIP_MAX_SIZE: usize = 2usize.pow(20);
pub const MAX_REQUEST_BLOCKS: usize = 2usize.pow(10);
pub const MIN_EPOCHS_FOR_BLOCK_REQUESTS: Epoch = 33024;
pub const MAX_CHUNK_SIZE: usize = 2usize.pow(20);
pub const TTFB_TIMEOUT: Duration = Duration::from_secs(5);
pub const RESP_TIMEOUT: Duration = Duration::from_secs(10);
pub const ATTESTATION_PROPAGATION_SLOT_RANGE: usize = 32;
pub const MAXIMUM_GOSSIP_CLOCK_DISPARITY: Duration = Duration::from_millis(500);
pub const MAX_INLINE_KEY_LENGTH: usize = 42;

// PeerId reimplemented from rust-libp2p
// revisit this implementation later
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeerId {
    multihash: Multihash,
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl PeerId {
    pub fn to_base58(&self) -> String {
        bs58::encode(self.multihash.to_bytes()).into_string()
    }
    /// Returns a raw bytes representation of this `PeerId`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.multihash.to_bytes()
    }
    /// Parses a `PeerId` from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<PeerId, Error> {
        PeerId::from_multihash(Multihash::from_bytes(data)?)
            .map_err(|mh| Error::UnsupportedCode(mh.code()))
    }
    /// Tries to turn a `Multihash` into a `PeerId`.
    pub fn from_multihash(multihash: Multihash) -> Result<PeerId, Multihash> {
        match Code::try_from(multihash.code()) {
            Ok(Code::Sha2_256) => Ok(PeerId { multihash }),
            Ok(Code::Identity) if multihash.digest().len() <= MAX_INLINE_KEY_LENGTH => {
                Ok(PeerId { multihash })
            }
            _ => Err(multihash),
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for PeerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_base58())
        } else {
            serializer.serialize_bytes(&self.to_bytes()[..])
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for PeerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::*;

        struct PeerIdVisitor;

        impl<'de> Visitor<'de> for PeerIdVisitor {
            type Value = PeerId;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "valid peer id")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                PeerId::from_bytes(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                PeerId::from_str(v).map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(PeerIdVisitor)
        } else {
            deserializer.deserialize_bytes(PeerIdVisitor)
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    B58,
    MultiHash,
}

impl From<bs58::decode::Error> for ParseError {
    fn from(_: bs58::decode::Error) -> Self {
        ParseError::B58
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::B58 => write!(f, "base-58 decode error"),
            ParseError::MultiHash => write!(f, "decoding multihash failed"),
        }
    }
}

impl FromStr for PeerId {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        PeerId::from_bytes(&bytes).map_err(|_| ParseError::MultiHash)
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

    #[test]
    fn test_id_str_format() {
        let id_repr = "QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N";
        let id: PeerId = PeerId::from_str(id_repr).unwrap();
        assert_eq!(
            format!("{id}"),
            "QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N"
        )
    }
}
