use std::borrow::Cow;

use crate::prelude::*;

pub trait EditorId {
    fn editor_id(&self) -> Cow<'_, str>;

    fn editor_id_ascii_lowercase(&self) -> Cow<'_, str> {
        match self.editor_id() {
            Cow::Borrowed(id) => id.cow_to_ascii_lowercase(),
            Cow::Owned(mut id) => {
                id.make_ascii_lowercase();
                id.into()
            }
        }
    }
}

impl EditorId for TES3Object {
    fn editor_id(&self) -> Cow<'_, str> {
        delegate! {
            match self {
                inner => inner.editor_id()
            }
        }
    }
}

impl EditorId for Header {
    fn editor_id(&self) -> Cow<'_, str> {
        "".into()
    }
}

impl EditorId for Skill {
    fn editor_id(&self) -> Cow<'_, str> {
        self.skill_id.display().into()
    }
}

impl EditorId for MagicEffect {
    fn editor_id(&self) -> Cow<'_, str> {
        self.effect_id.display().into()
    }
}

impl EditorId for Cell {
    fn editor_id(&self) -> Cow<'_, str> {
        if self.is_interior() {
            self.name.as_str().into()
        } else {
            let grid = self.data.grid;
            let name = if self.name.is_empty() { self.get_region() } else { &self.name };
            with_grid(name, grid).into()
        }
    }
}

impl EditorId for Landscape {
    fn editor_id(&self) -> Cow<'_, str> {
        with_grid(Self::TAG_STR, self.grid).into()
    }
}

impl EditorId for PathGrid {
    fn editor_id(&self) -> Cow<'_, str> {
        let grid = self.data.grid;
        let name = if self.cell.is_empty() { "Wilderness" } else { &self.cell };
        with_grid(name, grid).into()
    }
}

fn with_grid(name: &str, grid: (i32, i32)) -> String {
    let mut buffer = itoa::Buffer::new();
    let x = buffer.format(grid.0);
    let mut buffer = itoa::Buffer::new();
    let y = buffer.format(grid.1);
    [name, " (", x, ", ", y, ")"].concat()
}

macro_rules! impls {
    ($($T:ty)*) => {
        $(
            impl EditorId for $T {
                fn editor_id(&self) -> Cow<'_, str> {
                    self.id.as_str().into()
                }
            }
        )*
    }
}
impls! {
    // Header
    GameSetting
    GlobalVariable
    Class
    Faction
    Race
    Sound
    // Skill
    // MagicEffect
    Script
    Region
    Birthsign
    StartScript
    LandscapeTexture
    Spell
    Static
    Door
    MiscItem
    Weapon
    Container
    Creature
    Bodypart
    Light
    Enchanting
    Npc
    Armor
    Clothing
    RepairItem
    Activator
    Apparatus
    Lockpick
    Probe
    Ingredient
    Book
    Alchemy
    LeveledItem
    LeveledCreature
    // Cell
    // Landscape
    // PathGrid
    SoundGen
    Dialogue
    DialogueInfo
}
