pub mod traits;
pub use traits::*;

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
    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use smart_default::SmartDefault;
    pub(crate) use std::io;

    pub(crate) use slotmap::DefaultKey;
}
