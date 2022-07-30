#![cfg_attr(feature = "nightly", feature(drain_filter))]

pub mod types;
pub use types::*;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::*;

    // internal imports
    pub(crate) use bytes_io::*;
    pub(crate) use esp_macros::*;

    // external imports
    pub(crate) use bstr::{BString, ByteSlice, ByteVec};
    pub(crate) use enumflags2::{bitflags, BitFlag, BitFlags};
    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use smart_default::SmartDefault;

    // use [`std::default::default`] when stable
    pub(crate) fn default<T: Default>() -> T {
        Default::default()
    }
}
