// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Skill {
    pub flags1: u32,
    pub flags2: u32,
    pub skill_id: SkillId,
    pub data: Option<SkillData>,
    pub description: Option<String>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
pub struct SkillData {
    pub governing_attribute: i32,
    pub specialization: i32,
    pub actions: [f32; 4],
}

impl Load for Skill {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.skill_id = stream.load()?;
                }
                b"SKDT" => {
                    stream.expect(24u32)?;
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

impl Save for Skill {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // INDX
        stream.save(b"INDX")?;
        stream.save(&4u32)?;
        stream.save(&self.skill_id)?;
        // SKDT
        if let Some(value) = &self.data {
            stream.save(b"SKDT")?;
            stream.save(&24u32)?;
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
