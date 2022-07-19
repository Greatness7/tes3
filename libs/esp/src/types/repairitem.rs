// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct RepairItem {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub data: Option<RepairItemData>,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub icon: Option<String>,
    pub script: Option<String>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
pub struct RepairItemData {
    pub weight: f32,
    pub value: u32,
    pub uses: u32,
    pub quality: f32,
}

impl Load for RepairItem {
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
                b"RIDT" => {
                    stream.expect(16u32)?;
                    this.data = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"ITEX" => {
                    this.icon = Some(stream.load()?);
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

impl Save for RepairItem {
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
        // RIDT
        if let Some(value) = &self.data {
            stream.save(b"RIDT")?;
            stream.save(&16u32)?;
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
        // DELE
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}
