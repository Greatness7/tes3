// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct GlobalVariable {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub kind: Option<GlobalType>,
    pub value: Option<f32>,
    pub deleted: Option<u32>,
}

impl Load for GlobalVariable {
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
                    stream.expect(1u32)?;
                    this.kind = Some(stream.load()?);
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    this.value = Some(stream.load()?);
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

impl Save for GlobalVariable {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if let Some(value) = &self.kind {
            stream.save(b"FNAM")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        // FLTV
        if let Some(value) = &self.value {
            stream.save(b"FLTV")?;
            stream.save(&4u32)?;
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
