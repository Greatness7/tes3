// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Class {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub name: Option<String>,
    pub data: Option<ClassData>,
    pub description: Option<String>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassData {
    pub attribute1: AttributeId,
    pub attribute2: AttributeId,
    pub specialization: Specialization,
    pub minor1: SkillId,
    pub major1: SkillId,
    pub minor2: SkillId,
    pub major2: SkillId,
    pub minor3: SkillId,
    pub major3: SkillId,
    pub minor4: SkillId,
    pub major4: SkillId,
    pub minor5: SkillId,
    pub major5: SkillId,
    pub flags: u32,
    pub auto_calc_flags: u32,
}

impl Load for Class {
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
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"CLDT" => {
                    stream.expect(60u32)?;
                    this.data = Some(stream.load()?);
                }
                b"DESC" => {
                    this.description = Some(stream.load()?);
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

impl Save for Class {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // CLDT
        if let Some(value) = &self.data {
            stream.save(b"CLDT")?;
            stream.save(&60u32)?;
            stream.save(value)?;
        }
        // DESC
        if let Some(value) = &self.description {
            stream.save(b"DESC")?;
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
