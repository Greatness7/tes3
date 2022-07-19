#![feature(option_get_or_insert_default, default_free_fn, drain_filter)]

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
    pub(crate) use cow_utils::CowUtils;
    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use smart_default::SmartDefault;

    // helper functions
    pub(crate) use std::default::default;
}