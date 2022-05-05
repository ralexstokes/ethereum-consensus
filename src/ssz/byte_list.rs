use ssz_rs::prelude::*;
use std::fmt;
use std::hash::{self, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Eq, SimpleSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteList<const N: usize>(#[serde(with = "crate::serde::as_hex")] pub(crate) List<u8, N>);

impl<const N: usize> ByteList<N> {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, ssz_rs::DeserializeError> {
        ByteList::<N>::deserialize(bytes)
    }
}

// impl here to satisfy clippy
impl<const N: usize> PartialEq for ByteList<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> hash::Hash for ByteList<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<const N: usize> fmt::LowerHex for ByteList<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for i in &self.0[..] {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}

impl<const N: usize> fmt::Debug for ByteList<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteList<{}>({:x})", N, self)
    }
}

impl<const N: usize> fmt::Display for ByteList<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self)
    }
}

impl<const N: usize> AsRef<[u8]> for ByteList<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> Deref for ByteList<N> {
    type Target = List<u8, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteList<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
