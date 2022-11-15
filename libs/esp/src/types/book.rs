// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Book {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub icon: String,
    pub enchanting: String,
    pub text: String,
    pub data: BookData,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct BookData {
    pub weight: f32,
    pub value: u32,
    pub flags: u32,
    pub skill: SkillId,
    pub enchantment: u32,
}

impl Load for Book {
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
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"BKDT" => {
                    stream.expect(20u32)?;
                    this.data = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
                }
                b"TEXT" => {
                    this.text = stream.load()?;
                }
                b"ENAM" => {
                    this.enchanting = stream.load()?;
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

impl Save for Book {
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
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // BKDT
        stream.save(b"BKDT")?;
        stream.save(&20u32)?;
        stream.save(&self.data)?;
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // ITEX
        if !self.icon.is_empty() {
            stream.save(b"ITEX")?;
            stream.save(&self.icon)?;
        }
        // TEXT
        if !self.text.is_empty() {
            stream.save(b"TEXT")?;
            stream.save(&self.text)?;
        }
        // ENAM
        if !self.enchanting.is_empty() {
            stream.save(b"ENAM")?;
            stream.save(&self.enchanting)?;
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
