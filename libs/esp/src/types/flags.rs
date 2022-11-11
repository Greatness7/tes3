// internal imports
use crate::prelude::*;

// external imports
use bitflags::bitflags;

bitflags! {
    #[esp_meta]
    #[derive(LoadSave, Default)]
    #[repr(transparent)]
    pub struct ObjectFlags: u32 {
        const MODIFIED = 0x0002;
        const DELETED = 0x0020;
        const PERSISTENT = 0x0400;
        const IGNORED = 0x1000;
        const BLOCKED = 0x2000;
    }
}

bitflags! {
    #[esp_meta]
    #[derive(LoadSave, Default)]
    #[repr(transparent)]
    pub struct LandscapeFlags: u32 {
        const USES_VERTEX_HEIGHTS_AND_NORMALS = 0x01;
        const USES_VERTEX_COLORS = 0x02;
        const USES_TEXTURES = 0x04;
        const UNKNOWN = 0x08;
    }
}

bitflags! {
    #[esp_meta]
    #[derive(LoadSave, Default)]
    #[repr(transparent)]
    pub struct CellFlags: u32 {
        const IS_INTERIOR = 0x01;
        const HAS_WATER = 0x02;
        const RESTING_IS_ILLEGAL = 0x04;
        const BEHAVES_LIKE_EXTERIOR = 0x80;
    }
}
