// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct SoundGen {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub sound_flags: Option<u32>,
    pub creature: Option<String>,
    pub sound: Option<String>,
    pub deleted: Option<u32>,
}

impl Load for SoundGen {
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
                b"DATA" => {
                    stream.expect(4u32)?;
                    this.sound_flags = Some(stream.load()?);
                }
                b"CNAM" => {
                    this.creature = Some(stream.load()?);
                }
                b"SNAM" => {
                    this.sound = Some(stream.load()?);
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

impl Save for SoundGen {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        if let Some(value) = &self.sound_flags {
            stream.save(b"DATA")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.creature {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        // SNAM
        if let Some(value) = &self.sound {
            stream.save(b"SNAM")?;
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
