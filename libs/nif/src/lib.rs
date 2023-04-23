pub mod types;
pub use types::*;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::*;

    // internal imports
    pub(crate) use bytes_io::*;
    pub(crate) use nif_macros::*;

    // external imports
    pub(crate) use bstr::{BString, ByteSlice, ByteVec};
    pub(crate) use bytemuck::{Pod, Zeroable};
    pub(crate) use derive_more::{Deref, DerefMut, From, Into};
    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use smart_default::SmartDefault;
    pub(crate) use std::io;

    pub(crate) use slotmap::DefaultKey;

    // use [`std::default::default`] when stable
    pub(crate) fn default<T: Default>() -> T {
        Default::default()
    }
}
