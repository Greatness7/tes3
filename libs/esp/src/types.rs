use crate::prelude::*;

mod activator;
mod aidata;
mod aipackage;
mod alchemy;
mod apparatus;
mod armor;
mod bipedobject;
mod birthsign;
mod bodypart;
mod book;
mod cell;
mod class;
mod clothing;
mod container;
mod creature;
mod dialogue;
mod door;
mod effect;
mod enchanting;
mod enums;
mod faction;
mod flags;
mod gamesetting;
mod globalvariable;
mod header;
mod info;
mod ingredient;
mod landscape;
mod landscapetexture;
mod leveledcreature;
mod leveleditem;
mod light;
mod lockpick;
mod magiceffect;
mod miscitem;
mod npc;
mod pathgrid;
mod plugin;
mod probe;
mod race;
mod reference;
mod region;
mod repairitem;
mod script;
mod skill;
mod sound;
mod soundgen;
mod spell;
mod startscript;
mod static_;
mod string;
mod weapon;

pub use activator::*;
pub use aidata::*;
pub use aipackage::*;
pub use alchemy::*;
pub use apparatus::*;
pub use armor::*;
pub use bipedobject::*;
pub use birthsign::*;
pub use bodypart::*;
pub use book::*;
pub use cell::*;
pub use class::*;
pub use clothing::*;
pub use container::*;
pub use creature::*;
pub use dialogue::*;
pub use door::*;
pub use effect::*;
pub use enchanting::*;
pub use enums::*;
pub use faction::*;
pub use flags::*;
pub use gamesetting::*;
pub use globalvariable::*;
pub use header::*;
pub use info::*;
pub use ingredient::*;
pub use landscape::*;
pub use landscapetexture::*;
pub use leveledcreature::*;
pub use leveleditem::*;
pub use light::*;
pub use lockpick::*;
pub use magiceffect::*;
pub use miscitem::*;
pub use npc::*;
pub use pathgrid::*;
pub use plugin::*;
pub use probe::*;
pub use race::*;
pub use reference::*;
pub use region::*;
pub use repairitem::*;
pub use script::*;
pub use skill::*;
pub use sound::*;
pub use soundgen::*;
pub use spell::*;
pub use startscript::*;
pub use static_::*;
pub use string::*;
pub use weapon::*;

#[rustfmt::skip]
#[esp_meta]
#[derive(TES3Object, Clone, Debug, PartialEq)]
pub enum TES3Object {
    #[tag(b"TES3")] Header(Header),
    #[tag(b"GMST")] GameSetting(GameSetting),
    #[tag(b"GLOB")] GlobalVariable(GlobalVariable),
    #[tag(b"CLAS")] Class(Class),
    #[tag(b"FACT")] Faction(Faction),
    #[tag(b"RACE")] Race(Race),
    #[tag(b"SOUN")] Sound(Sound),
    #[tag(b"SKIL")] Skill(Skill),
    #[tag(b"MGEF")] MagicEffect(MagicEffect),
    #[tag(b"SCPT")] Script(Script),
    #[tag(b"REGN")] Region(Region),
    #[tag(b"BSGN")] Birthsign(Birthsign),
    #[tag(b"SSCR")] StartScript(StartScript),
    #[tag(b"LTEX")] LandscapeTexture(LandscapeTexture),
    #[tag(b"SPEL")] Spell(Spell),
    #[tag(b"STAT")] Static(Static),
    #[tag(b"DOOR")] Door(Door),
    #[tag(b"MISC")] MiscItem(MiscItem),
    #[tag(b"WEAP")] Weapon(Weapon),
    #[tag(b"CONT")] Container(Container),
    #[tag(b"CREA")] Creature(Creature),
    #[tag(b"BODY")] Bodypart(Bodypart),
    #[tag(b"LIGH")] Light(Light),
    #[tag(b"ENCH")] Enchanting(Enchanting),
    #[tag(b"NPC_")] Npc(Npc),
    #[tag(b"ARMO")] Armor(Armor),
    #[tag(b"CLOT")] Clothing(Clothing),
    #[tag(b"REPA")] RepairItem(RepairItem),
    #[tag(b"ACTI")] Activator(Activator),
    #[tag(b"APPA")] Apparatus(Apparatus),
    #[tag(b"LOCK")] Lockpick(Lockpick),
    #[tag(b"PROB")] Probe(Probe),
    #[tag(b"INGR")] Ingredient(Ingredient),
    #[tag(b"BOOK")] Book(Book),
    #[tag(b"ALCH")] Alchemy(Alchemy),
    #[tag(b"LEVI")] LeveledItem(LeveledItem),
    #[tag(b"LEVC")] LeveledCreature(LeveledCreature),
    #[tag(b"CELL")] Cell(Cell),
    #[tag(b"LAND")] Landscape(Landscape),
    #[tag(b"PGRD")] PathGrid(PathGrid),
    #[tag(b"SNDG")] SoundGen(SoundGen),
    #[tag(b"DIAL")] Dialogue(Dialogue),
    #[tag(b"INFO")] Info(Info),
}
