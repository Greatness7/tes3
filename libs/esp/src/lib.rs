#![cfg_attr(feature = "nightly", feature(drain_filter))]

pub mod types;
pub use types::*;

pub mod traits;
pub use traits::*;

#[cfg(feature = "serde")]
pub(crate) mod features;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::*;

    // internal imports
    pub(crate) use bytes_io::*;
    pub(crate) use esp_macros::*;

    // external imports
    pub(crate) use bstr::{BString, ByteSlice, ByteVec};
    pub(crate) use cow_utils::CowUtils;
    pub(crate) use derive_more::{Deref, DerefMut, From, Into};
    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use smart_default::SmartDefault;
    pub(crate) use std::io;

    // use [`std::default::default`] when stable
    pub(crate) fn default<T: Default>() -> T {
        Default::default()
    }
}
