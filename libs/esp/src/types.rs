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
#[derive(TES3Object, Clone, Debug, From, PartialEq)]
pub enum TES3Object {
    #[tag("TES3")] Header(Header),
    #[tag("GMST")] GameSetting(GameSetting),
    #[tag("GLOB")] GlobalVariable(GlobalVariable),
    #[tag("CLAS")] Class(Class),
    #[tag("FACT")] Faction(Faction),
    #[tag("RACE")] Race(Race),
    #[tag("SOUN")] Sound(Sound),
    #[tag("SNDG")] SoundGen(SoundGen),
    #[tag("SKIL")] Skill(Skill),
    #[tag("MGEF")] MagicEffect(MagicEffect),
    #[tag("SCPT")] Script(Script),
    #[tag("REGN")] Region(Region),
    #[tag("BSGN")] Birthsign(Birthsign),
    #[tag("SSCR")] StartScript(StartScript),
    #[tag("LTEX")] LandscapeTexture(LandscapeTexture),
    #[tag("SPEL")] Spell(Spell),
    #[tag("STAT")] Static(Static),
    #[tag("DOOR")] Door(Door),
    #[tag("MISC")] MiscItem(MiscItem),
    #[tag("WEAP")] Weapon(Weapon),
    #[tag("CONT")] Container(Container),
    #[tag("CREA")] Creature(Creature),
    #[tag("BODY")] Bodypart(Bodypart),
    #[tag("LIGH")] Light(Light),
    #[tag("ENCH")] Enchanting(Enchanting),
    #[tag("NPC_")] Npc(Npc),
    #[tag("ARMO")] Armor(Armor),
    #[tag("CLOT")] Clothing(Clothing),
    #[tag("REPA")] RepairItem(RepairItem),
    #[tag("ACTI")] Activator(Activator),
    #[tag("APPA")] Apparatus(Apparatus),
    #[tag("LOCK")] Lockpick(Lockpick),
    #[tag("PROB")] Probe(Probe),
    #[tag("INGR")] Ingredient(Ingredient),
    #[tag("BOOK")] Book(Book),
    #[tag("ALCH")] Alchemy(Alchemy),
    #[tag("LEVI")] LeveledItem(LeveledItem),
    #[tag("LEVC")] LeveledCreature(LeveledCreature),
    #[tag("CELL")] Cell(Cell),
    #[tag("LAND")] Landscape(Landscape),
    #[tag("PGRD")] PathGrid(PathGrid),
    #[tag("DIAL")] Dialogue(Dialogue),
    #[tag("INFO")] Info(Info),
}
