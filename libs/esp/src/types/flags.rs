// internal imports
use crate::prelude::*;

#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectFlags {
    Deleted = 0x0020,
    Persistent = 0x0400,
    Ignored = 0x1000,
    Blocked = 0x2000,
}

#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LandscapeFlags {
    UsesVertexHeightsAndNormals = 0x01,
    UsesVertexColors = 0x02,
    UsesTextures = 0x04,
    Unknown = 0x08,
}
