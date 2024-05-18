// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Alchemy {
    pub flags: ObjectFlags,
    pub id: String,
    pub data: Option<AlchemyData>,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub icon: Option<String>,
    pub script: Option<String>,
    pub effects: Option<Vec<Effect>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
pub struct AlchemyData {
    pub weight: f32,
    pub value: u32,
    pub flags: u32,
}

impl Load for Alchemy {
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
                b"TEXT" => {
                    this.icon = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"ALDT" => {
                    stream.expect(12u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ENAM" => {
                    stream.expect(24u32)?;
                    this.effects.get_or_insert_with(default).push(stream.load()?);
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

impl Save for Alchemy {
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
        // TEXT
        if let Some(value) = &self.icon {
            stream.save(b"TEXT")?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // ALDT
        if let Some(value) = &self.data {
            stream.save(b"ALDT")?;
            stream.save(&12u32)?;
            stream.save(value)?;
        }
        // ENAM
        for effect in self.effects.iter().flatten() {
            stream.save(b"ENAM")?;
            stream.save(&24u32)?;
            stream.save(effect)?;
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
