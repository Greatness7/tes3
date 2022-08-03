// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct SoundGen {
    pub flags: ObjectFlags,
    pub id: String,
    pub sound_flags: Option<u32>,
    pub creature: Option<String>,
    pub sound: Option<String>,
}

impl Load for SoundGen {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

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

impl Save for SoundGen {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
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
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
