pub mod types;
pub use types::*;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::*;

    // internal imports
    pub use bytes_io::*;
    pub use nif_macros::*;

    // external imports
    pub use bstr::{BString, ByteSlice, ByteVec};
    pub use bytemuck::{Pod, Zeroable};
    pub use derive_more::{Deref, DerefMut, From, Into};
    pub use hashbrown::{HashMap, HashSet};
    pub use smart_default::SmartDefault;
    pub use std::io;

    // use [`std::default::default`] when stable
    pub fn default<T: Default>() -> T {
        Default::default()
    }

    // basic math types
    mod math {
        pub use glam::{Mat2, Mat3, Quat, Vec2, Vec3, Vec4};
        // a temporary alias until we pick a color library
        pub type ColorA = Vec4;
    }
    pub use math::*;
}
