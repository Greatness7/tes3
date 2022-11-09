// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SoundGen {
    pub flags: ObjectFlags,
    pub id: String,
    pub sound_flags: u32,
    pub creature: String,
    pub sound: String,
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
                    this.sound_flags = stream.load()?;
                }
                b"CNAM" => {
                    this.creature = stream.load()?;
                }
                b"SNAM" => {
                    this.sound = stream.load()?;
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
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.sound_flags)?;
        // CNAM
        if !self.creature.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.creature)?;
        }
        // SNAM
        if !self.sound.is_empty() {
            stream.save(b"SNAM")?;
            stream.save(&self.sound)?;
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
