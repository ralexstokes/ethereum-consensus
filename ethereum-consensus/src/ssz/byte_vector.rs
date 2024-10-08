use crate::{
    serde::{write_bytes_to_lower_hex, write_bytes_to_lower_hex_display},
    ssz::prelude::*,
};
use alloy_primitives::FixedBytes;
use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

#[derive(
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    SimpleSerialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ByteVector<const N: usize>(#[serde(with = "crate::serde::as_hex")] Vector<u8, N>);

impl<const N: usize> TryFrom<&[u8]> for ByteVector<N> {
    type Error = DeserializeError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        ByteVector::<N>::deserialize(bytes)
    }
}

impl<const N: usize> TryFrom<FixedBytes<N>> for ByteVector<N> {
    type Error = DeserializeError;

    fn try_from(bytes: FixedBytes<N>) -> Result<Self, Self::Error> {
        bytes.as_slice().try_into()
    }
}

impl<const N: usize> From<ByteVector<N>> for FixedBytes<N> {
    fn from(bytes: ByteVector<N>) -> Self {
        FixedBytes::from_slice(bytes.as_ref())
    }
}

impl<const N: usize> Hash for ByteVector<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<const N: usize> fmt::Debug for ByteVector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_bytes_to_lower_hex(f, &self.0)
    }
}

impl<const N: usize> fmt::Display for ByteVector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_bytes_to_lower_hex_display(f, self.0.iter())
    }
}

impl<const N: usize> AsRef<[u8]> for ByteVector<N> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> Deref for ByteVector<N> {
    type Target = Vector<u8, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteVector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test conversion between ByteVector and FixedBytes
    #[test]
    fn test_byte_vector_fixed_bytes_conversion() {
        let bytes = [255u8; 32];
        let byte_vector = ByteVector::<32>::try_from(&bytes[..]).unwrap();
        let fixed_bytes = FixedBytes::from_slice(&bytes);
        assert_eq!(byte_vector, fixed_bytes.try_into().unwrap());
        assert_eq!(fixed_bytes, FixedBytes::from(byte_vector));
    }
}
