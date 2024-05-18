// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Armor {
    pub flags: ObjectFlags,
    pub id: String,
    pub data: Option<ArmorData>,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub icon: Option<String>,
    pub script: Option<String>,
    pub enchanting: Option<String>,
    pub biped_objects: Option<Vec<BipedObject>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
pub struct ArmorData {
    pub kind: ArmorType,
    pub weight: f32,
    pub value: u32,
    pub health: u32,
    pub enchantment: u32,
    pub armor_rating: u32,
}

impl Load for Armor {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"AODT" => {
                    stream.expect(24u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ITEX" => {
                    this.icon = Some(stream.load()?);
                }
                b"INDX" => {
                    this.biped_objects.get_or_insert_with(default).push(stream.load()?);
                }
                b"ENAM" => {
                    this.enchanting = Some(stream.load()?);
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::DELETED);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Armor {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if let Some(value) = &self.mesh {
            stream.save(b"MODL")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // AODT
        if let Some(value) = &self.data {
            stream.save(b"AODT")?;
            stream.save(&24u32)?;
            stream.save(value)?;
        }
        // ITEX
        if let Some(value) = &self.icon {
            stream.save(b"ITEX")?;
            stream.save(value)?;
        }
        // INDX / BNAM / CNAM
        for biped_object in self.biped_objects.iter().flatten() {
            stream.save(biped_object)?;
        }
        // ENAM
        if let Some(value) = &self.enchanting {
            stream.save(b"ENAM")?;
            stream.save(value)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
