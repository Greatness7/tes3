#![cfg_attr(feature = "nightly", feature(extract_if))]

pub mod types;
pub use types::*;

pub mod traits;
pub use traits::*;

pub(crate) mod features;
pub(crate) mod macros;

#[cfg(feature = "lua")]
pub use features::lua::lua_module;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::*;

    // internal imports
    pub use bytes_io::*;
    pub use esp_macros::*;

    // external imports
    pub use bstr::{BString, ByteSlice, ByteVec};
    pub use cow_utils::CowUtils;
    pub use derive_more::{Deref, DerefMut, From, Into};
    pub use hashbrown::HashSet;
    pub use smart_default::SmartDefault;
    pub use std::io;

    pub type HashMap<K, V> = indexmap::IndexMap<K, V, hashbrown::hash_map::DefaultHashBuilder>;

    // use [`std::default::default`] when stable
    pub fn default<T: Default>() -> T {
        Default::default()
    }
}
