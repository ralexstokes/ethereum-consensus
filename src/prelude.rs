mod core {
    #[cfg(not(feature = "std"))]
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
pub use tokio::time::Duration;
pub use tokio::time::Instant;

pub use self::iter::Enumerate;

#[cfg(not(feature = "std"))]
pub use alloc::boxed::Box;

pub use alloc::string::String;
#[cfg(not(feature = "std"))]
pub use alloc::{vec, vec::Vec};
pub use hashbrown::{HashMap, HashSet};

pub use alloc::format;
pub use alloc::str::FromStr;
pub use alloc::string::ToString;
pub use alloc::sync::Arc;
