// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct Effect {
    pub magic_effect: EffectId2,
    pub skill: SkillId2,
    pub attribute: AttributeId2,
    pub range: EffectRange,
    pub area: u32,
    pub duration: u32,
    pub min_magnitude: u32,
    pub max_magnitude: u32,
}
