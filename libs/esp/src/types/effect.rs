// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct Effect {
    pub magic_effect: EffectId2,
    pub skill: SkillId2,
    pub attribute: AttributeId2,
    pub range: u32,
    pub area: u32,
    pub duration: u32,
    pub min_magnitude: u32,
    pub max_magnitude: u32,
}
