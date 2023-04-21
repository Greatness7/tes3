pub type Vec2 = [f32; 2];
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];

pub type Quat = [f32; 4];

pub type Color = [f32; 3];
pub type ColorA = [f32; 4];

pub type Mat2 = [[f32; 2]; 2];
pub type Mat3 = [[f32; 3]; 3];
pub type Mat4 = [[f32; 4]; 4];

#[rustfmt::skip]
pub(crate) const QUAT_IDENTITY: Quat = {
    [0.0, 0.0, 0.0, 1.0]
};

#[rustfmt::skip]
pub(crate) const MAT2_IDENTITY: Mat2 = [
    [1.0, 0.0],
    [0.0, 1.0]
];

#[rustfmt::skip]
pub(crate) const MAT3_IDENTITY: Mat3 = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0]
];
