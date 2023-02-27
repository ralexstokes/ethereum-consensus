#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod altair;
pub mod bellatrix;
pub mod builder;
pub mod capella;
#[cfg(feature = "std")]
pub mod clock;
pub mod configs;
pub mod crypto;
pub mod domains;
#[cfg(feature = "std")]
pub mod networking;
pub mod phase0;
pub mod primitives;
#[cfg(feature = "serde")]
pub mod serde;
pub mod signing;
pub mod ssz;
pub mod state_transition;

mod lib {
    mod core {
        pub use core::*;
    }

    pub use self::core::{any, cmp, iter, slice};

    pub use self::cmp::Ordering;
    pub use self::core::array::TryFromSliceError;
    pub use self::core::convert::TryFrom;
    pub use self::core::fmt::{self, Debug, Display, Formatter};
    pub use self::core::hash::{Hash, Hasher};
    pub use self::core::iter::repeat;
    pub use self::core::iter::zip;
    pub use self::core::marker::PhantomData;
    pub use self::core::mem;
    pub use self::core::ops::{Deref, DerefMut, Index, IndexMut};
    pub use self::core::option::Option;
    pub use self::core::slice::{IterMut, SliceIndex};
    #[cfg(feature = "std")]
    pub use tokio::time::Duration;
    #[cfg(feature = "std")]
    pub use tokio::time::Instant;

    pub use self::iter::Enumerate;

    #[cfg(not(feature = "std"))]
    pub use alloc::boxed::Box;

    pub use alloc::format;
    pub use alloc::str::FromStr;
    #[cfg(not(feature = "std"))]
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::sync::Arc;
    #[cfg(not(feature = "std"))]
    pub use alloc::{vec, vec::Vec};
    pub use hashbrown::{HashMap, HashSet};
}
