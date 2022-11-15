#![cfg_attr(feature = "nightly", feature(drain_filter))]

pub mod types;
pub use types::*;

#[allow(unused_imports, unreachable_pub)]
pub(crate) mod prelude {
    pub use super::*;

    // internal imports
    pub use bytes_io::*;
    pub use esp_macros::*;

    // external imports
    pub use bstr::{BString, ByteSlice, ByteVec};
    pub use hashbrown::{HashMap, HashSet};
    pub use smart_default::SmartDefault;
    pub use std::io;

    // use [`std::default::default`] when stable
    pub fn default<T: Default>() -> T {
        Default::default()
    }
}
