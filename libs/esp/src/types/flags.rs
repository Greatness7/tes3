// internal imports
use crate::prelude::*;

// external imports
use bitflags::bitflags;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

bitflags! {
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    #[derive(LoadSave, Default)]
    #[repr(transparent)]
    pub struct ObjectFlags: u32 {
        const DELETED = 0x0020;
        const PERSISTENT = 0x0400;
        const IGNORED = 0x1000;
        const BLOCKED = 0x2000;
    }
}

bitflags! {
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    #[derive(LoadSave, Default)]
    #[repr(transparent)]
    pub struct LandscapeFlags: u32 {
        const USES_VERTEX_HEIGHTS_AND_NORMALS = 0x01;
        const USES_VERTEX_COLORS = 0x02;
        const USES_TEXTURES = 0x04;
        const UNKNOWN = 0x08;
    }
}
