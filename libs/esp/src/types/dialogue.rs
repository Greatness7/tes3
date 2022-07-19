// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Dialogue {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub kind: Option<DialogueType2>,
    pub deleted: Option<u32>,
}

impl Load for Dialogue {
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
                    // When the dialogue is marked as deleted this field (sometimes) has size 4
                    let size: u32 = stream.load()?;
                    if size == 1 {
                        this.kind = Some(stream.load()?);
                    } else {
                        stream.skip(size as _)?;
                    }
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

impl Save for Dialogue {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        if let Some(value) = &self.kind {
            stream.save(b"DATA")?;
            stream.save(&1u32)?;
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
