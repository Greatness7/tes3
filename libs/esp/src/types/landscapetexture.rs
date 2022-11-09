// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LandscapeTexture {
    pub flags: ObjectFlags,
    pub id: String,
    pub index: u32,
    pub file_name: String,
}

impl Load for LandscapeTexture {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    this.index = stream.load()?;
                }
                b"DATA" => {
                    this.file_name = stream.load()?;
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

impl Save for LandscapeTexture {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // INTV
        stream.save(b"INTV")?;
        stream.save(&4u32)?;
        stream.save(&self.index)?;
        // DATA
        if !self.file_name.is_empty() {
            stream.save(b"DATA")?;
            stream.save(&self.file_name)?;
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
