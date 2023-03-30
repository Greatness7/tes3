// internal imports
use crate::prelude::*;

// external imports
use nalgebra::{Const, Dyn, OMatrix, SVector};

#[derive(Clone, Debug, Deref, DerefMut, From, PartialEq, SmartDefault)]
pub struct NiAnimationKey<const KEY_SIZE: usize, const VALUE_SIZE: usize> {
    #[default(SVector::<_, KEY_SIZE>::zeros())]
    pub data: SVector<f32, KEY_SIZE>,
}

#[derive(Clone, Debug, Deref, DerefMut, From, PartialEq, SmartDefault)]
pub struct NiAnimationKeys<const KEY_SIZE: usize, const VALUE_SIZE: usize> {
    #[default(Empty::empty())]
    pub data: OMatrix<f32, Const<KEY_SIZE>, Dyn>,
}

#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinFloatKey(NiAnimationKey<2, 1>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezFloatKey(NiAnimationKey<4, 1>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBFloatKey(NiAnimationKey<5, 1>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinPosKey(NiAnimationKey<4, 3>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezPosKey(NiAnimationKey<10, 3>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBPosKey(NiAnimationKey<7, 3>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinColKey(NiAnimationKey<5, 4>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinRotKey(NiAnimationKey<5, 4>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezRotKey(NiAnimationKey<5, 4>);
#[doc(hidden)]
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBRotKey(NiAnimationKey<8, 4>);

#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinFloatKeys(NiAnimationKeys<2, 1>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezFloatKeys(NiAnimationKeys<4, 1>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBFloatKeys(NiAnimationKeys<5, 1>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinPosKeys(NiAnimationKeys<4, 3>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezPosKeys(NiAnimationKeys<10, 3>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBPosKeys(NiAnimationKeys<7, 3>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinColKeys(NiAnimationKeys<5, 4>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct LinRotKeys(NiAnimationKeys<5, 4>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct BezRotKeys(NiAnimationKeys<5, 4>);
#[derive(Clone, Debug, Default, Deref, DerefMut, From, PartialEq)]
pub struct TCBRotKeys(NiAnimationKeys<8, 4>);
