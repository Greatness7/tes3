pub mod traits;
pub use traits::*;

pub mod types;
pub use types::*;

#[allow(unused_imports, unreachable_pub)]
pub(crate) mod prelude {
    pub use super::*;

    // internal imports
    pub use bytes_io::*;
    pub use nif_macros::*;

    // external imports
    pub use bstr::{BString, ByteSlice, ByteVec};
    pub use hashbrown::{HashMap, HashSet};
    pub use smart_default::SmartDefault;
    pub use std::io;

    pub use slotmap::DefaultKey;
}
