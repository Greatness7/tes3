// internal imports
use crate::prelude::*;

// external imports
use bitflags::bitflags;

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct ObjectFlags: u32 {
        const MODIFIED = 0x2;
        const DELETED = 0x20;
        const PERSISTENT = 0x400;
        const IGNORED = 0x1000;
        const BLOCKED = 0x2000;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct LandscapeFlags: u32 {
        const USES_VERTEX_HEIGHTS_AND_NORMALS = 0x1;
        const USES_VERTEX_COLORS = 0x2;
        const USES_TEXTURES = 0x4;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct CellFlags: u32 {
        const IS_INTERIOR = 0x1;
        const HAS_WATER = 0x2;
        const RESTING_IS_ILLEGAL = 0x4;
        const BEHAVES_LIKE_EXTERIOR = 0x80;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct LightFlags: u32 {
        const DYNAMIC = 0x1;
        const CAN_CARRY = 0x2;
        const NEGATIVE = 0x4;
        const FLICKER = 0x8;
        const FIRE = 0x10;
        const OFF_BY_DEFAULT = 0x20;
        const FLICKER_SLOW = 0x40;
        const PULSE = 0x80;
        const PULSE_SLOW = 0x100;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct MiscItemFlags: u32 {
        const KEY = 0x1;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct AlchemyFlags: u32 {
        const AUTO_CALCULATE = 0x1;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct EnchantingFlags: u32 {
        const AUTO_CALCULATE = 0x1;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct SpellFlags: u32 {
        const AUTO_CALCULATE = 0x1;
        const PC_START_SPELL = 0x2;
        const ALWAYS_SUCCEEDS = 0x4;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct WeaponFlags: u32 {
        const IGNORES_NORMAL_WEAPON_RESISTANCE = 0x1;
        const SILVER = 0x2;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct ClassFlags: u32 {
        const PLAYABLE = 0x1;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct FactionFlags: u32 {
        const HIDDEN_FROM_PC = 0x1;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct RaceFlags: u32 {
        const PLAYABLE = 0x1;
        const BEAST_RACE = 0x2;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct MagicEffectFlags: u32 {
        const TARGET_SKILL = 0x1;
        const TARGET_ATTRIBUTE = 0x2;
        const NO_DURATION = 0x4;
        const NO_MAGNITUDE = 0x8;
        const HARMFUL = 0x10;
        const CONTINUOUS_VFX = 0x20;
        const CAN_CAST_SELF = 0x40;
        const CAN_CAST_TOUCH = 0x80;
        const CAN_CAST_TARGET = 0x100;
        const ALLOW_SPELLMAKING = 0x200;
        const ALLOW_ENCHANTING = 0x400;
        const NEGATIVE_LIGHTING = 0x800;
        const APPLIED_ONCE = 0x1000;
        const UNKNOWN_CHAMELEON = 0x2000;
        const NON_RECASTABLE = 0x4000;
        const ILLEGAL_DAEDRA = 0x8000;
        const UNREFLECTABLE = 0x10000;
        const CASTER_LINKED = 0x20000;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct ContainerFlags: u32 {
        const ORGANIC = 0x1;
        const RESPAWNS = 0x2;
        const IS_BASE = 0x8;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct CreatureFlags: u8 {
        const BIPED = 0x1;
        const RESPAWN = 0x2;
        const WEAPON_AND_SHIELD = 0x4;
        const IS_BASE = 0x8;
        const SWIMS = 0x10;
        const FLIES = 0x20;
        const WALKS = 0x40;
        const ESSENTIAL = 0x80;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct NpcFlags: u8 {
        const FEMALE = 0x1;
        const ESSENTIAL = 0x2;
        const RESPAWN = 0x4;
        const IS_BASE = 0x8;
        const AUTO_CALCULATE = 0x10;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct ServiceFlags: u32 {
        const BARTERS_WEAPONS = 0x1;
        const BARTERS_ARMOR = 0x2;
        const BARTERS_CLOTHING = 0x4;
        const BARTERS_BOOKS = 0x8;
        const BARTERS_INGREDIENTS = 0x10;
        const BARTERS_LOCKPICKS = 0x20;
        const BARTERS_PROBES = 0x40;
        const BARTERS_LIGHTS = 0x80;
        const BARTERS_APPARATUS = 0x100;
        const BARTERS_REPAIR_TOOLS = 0x200;
        const BARTERS_MISC_ITEMS = 0x400;
        const OFFERS_SPELLS = 0x800;
        const BARTERS_ENCHANTED_ITEMS = 0x1000;
        const BARTERS_ALCHEMY = 0x2000;
        const OFFERS_TRAINING = 0x4000;
        const OFFERS_SPELLMAKING = 0x8000;
        const OFFERS_ENCHANTING = 0x10000;
        const OFFERS_REPAIRS = 0x20000;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct LeveledItemFlags: u32 {
        const CALCULATE_FOR_EACH_ITEM = 0x1;
        const CALCULATE_FROM_ALL_LEVELS = 0x2;
    }
}

bitflags! {
    #[esp_meta]
    #[repr(transparent)]
    #[derive(LoadSave, Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct LeveledCreatureFlags: u32 {
        const CALCULATE_FROM_ALL_LEVELS = 0x1;
    }
}
