// internal imports
use crate::prelude::*;

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum BoundType {
    #[default]
    Sphere = 0,
    Box = 1,
    Capsule = 2,
    Lozenge = 3,
    Union = 4,
    Halfspace = 5,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum KeyContent {
    #[default]
    FloatKey = 0,
    PosKey = 1,
    RotKey = 2,
    ColorKey = 3,
    TextKey = 4,
    VisKey = 5,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum KeyType {
    #[default]
    NoInterp = 0,
    LinKey = 1,
    BezKey = 2,
    TCBKey = 3,
    EulerKey = 4,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum ForceType {
    #[default]
    Planar = 0,
    Spherical = 1,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum ColorField {
    #[default]
    Ambient = 0,
    Diffuse = 1,
    Specular = 2,
    Emissive = 3,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum DecayType {
    #[default]
    None = 0,
    Linear = 1,
    Exponential = 2,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum SymmetryType {
    #[default]
    Spherical = 0,
    Cylindrical = 1,
    Planar = 2,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum BankDirection {
    Negative = -1,
    #[default]
    Positive = 1,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum PixelFormat {
    #[default]
    RGB = 0,
    RGBA = 1,
    PAL = 2,
    PALAlpha = 3,
    Compress1 = 4,
    Compress3 = 5,
    Compress5 = 6,
    RGB24NonInterleaved = 7,
    Bump = 8,
    BumpLuma = 9,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum AxisOrder {
    #[default]
    XYZ = 0,
    XZY = 1,
    YZX = 2,
    YXZ = 3,
    ZXY = 4,
    ZYX = 5,
    XYX = 6,
    YZY = 7,
    ZXZ = 8,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum SortingMode {
    #[default]
    Inherit = 0,
    Off = 1,
    Subsort = 2,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum PixelLayout {
    Palettized8 = 0,
    HighColor16 = 1,
    TrueColor32 = 2,
    Compressed = 3,
    BumpMap = 4,
    Palettized4 = 5,
    #[default]
    Default = 6,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum UseMipMaps {
    No = 0,
    Yes = 1,
    #[default]
    Default = 2,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum AlphaFormat {
    None = 0,
    Binary = 1,
    Smooth = 2,
    #[default]
    Default = 3,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum TestFunction {
    #[default]
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum Action {
    #[default]
    Keep = 0,
    Zero = 1,
    Replace = 2,
    Increment = 3,
    Decrement = 4,
    Invert = 5,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum DrawMode {
    #[default]
    Default = 0,
    CounterClockwise = 1,
    Clockwise = 2,
    Both = 3,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum TextureType {
    #[default]
    ProjectedLight = 0,
    ProjectedShadow = 1,
    EnvironmentMap = 2,
    FogMap = 3,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum CoordGenType {
    #[default]
    WorldParallel = 0,
    WorldPerspective = 1,
    SphereMap = 2,
    SpecularCubeMap = 3,
    DiffuseCubeMap = 4,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum ClampMode {
    ClampSClampT = 0,
    ClampSWrapT = 1,
    WrapSClampT = 2,
    #[default]
    WrapSWrapT = 3,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum FilterMode {
    Nearest = 0,
    Bilerp = 1,
    #[default]
    Trilerp = 2,
    NearestMipNearest = 3,
    NearestMipLerp = 4,
    BilerpMipNearest = 5,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum ApplyMode {
    Replace = 0,
    Decal = 1,
    #[default]
    Modulate = 2,
    Hilight = 3,
    Hilight2 = 4,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum SourceVertexMode {
    #[default]
    Ignore = 0,
    Emissive = 1,
    AmbientDiffuse = 2,
}

#[derive(LoadSave, Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(i32)]
pub enum LightingMode {
    Emissive = 0,
    #[default]
    EmissiveAmbientDiffuse = 1,
}
