// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Clothing {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub data: Option<ClothingData>,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub icon: Option<String>,
    pub script: Option<String>,
    pub enchanting: Option<String>,
    pub biped_objects: Option<Vec<BipedObject>>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
pub struct ClothingData {
    pub kind: ClothingType,
    pub weight: f32,
    pub value: u16,
    pub enchantment: u16,
}

impl Load for Clothing {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

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
                b"CTDT" => {
                    stream.expect(12u32)?;
                    this.data = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"ITEX" => {
                    this.icon = Some(stream.load()?);
                }
                b"INDX" => {
                    this.biped_objects.get_or_insert_default().push(stream.load()?);
                }
                b"ENAM" => {
                    this.enchanting = Some(stream.load()?);
                }
                b"DELE" => {
                    stream.expect(4u32)?;
                    this.deleted = Some(stream.load()?);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Clothing {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
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
        // CTDT
        if let Some(value) = &self.data {
            stream.save(b"CTDT")?;
            stream.save(&12u32)?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
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
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}
