// external imports
use bytemuck::{Pod, Zeroable};

// internal imports
use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiLinFloatKey {
    time: f32,
    value: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiBezFloatKey {
    time: f32,
    value: f32,
    in_tan: f32,
    out_an: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiTCBFloatKey {
    time: f32,
    value: f32,
    t: f32,
    c: f32,
    b: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiLinPosKey {
    time: f32,
    value: Vec3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiBezPosKey {
    time: f32,
    value: Vec3,
    in_tan: Vec3,
    out_an: Vec3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiTCBPosKey {
    time: f32,
    value: Vec3,
    t: f32,
    c: f32,
    b: f32,
}

#[derive(LoadSave, Clone, Copy, Debug, Default, PartialEq, Zeroable)]
pub struct NiLinColKey {
    time: f32,
    value: ColorA,
}

#[derive(LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiLinRotKey {
    time: f32,
    #[default(QUAT_IDENTITY)]
    value: Quat,
}

#[derive(LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiBezRotKey {
    time: f32,
    #[default(QUAT_IDENTITY)]
    value: Quat,
}

#[derive(LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiTCBRotKey {
    time: f32,
    #[default(QUAT_IDENTITY)]
    value: Quat,
    t: f32,
    c: f32,
    b: f32,
}
