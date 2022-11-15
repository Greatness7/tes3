// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Alchemy {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub effects: Vec<Effect>,
    pub data: AlchemyData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
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
                    this.mesh = stream.load()?;
                }
                b"TEXT" => {
                    this.icon = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"ALDT" => {
                    stream.expect(12u32)?;
                    this.data = stream.load()?;
                }
                b"ENAM" => {
                    stream.expect(24u32)?;
                    this.effects.push(stream.load()?);
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
        if !self.mesh.is_empty() {
            stream.save(b"MODL")?;
            stream.save(&self.mesh)?;
        }
        // TEXT
        if !self.icon.is_empty() {
            stream.save(b"TEXT")?;
            stream.save(&self.icon)?;
        }
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // ALDT
        stream.save(b"ALDT")?;
        stream.save(&12u32)?;
        stream.save(&self.data)?;
        // ENAM
        for value in &self.effects {
            stream.save(b"ENAM")?;
            stream.save(&24u32)?;
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
