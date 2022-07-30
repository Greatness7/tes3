// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Dialogue {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub kind: Option<DialogueType2>,
}

impl Load for Dialogue {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

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
                        stream.skip(size)?;
                    }
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::Deleted);
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
        stream.save(&self.flags)?;
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
        if self.flags.contains(ObjectFlags::Deleted) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
