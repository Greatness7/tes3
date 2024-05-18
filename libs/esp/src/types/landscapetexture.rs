// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct LandscapeTexture {
    pub flags: ObjectFlags,
    pub id: String,
    pub index: Option<u32>,
    pub file_name: Option<String>,
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
                    this.index = Some(stream.load()?);
                }
                b"DATA" => {
                    this.file_name = Some(stream.load()?);
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
        if let Some(value) = &self.index {
            stream.save(b"INTV")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // DATA
        if let Some(value) = &self.file_name {
            stream.save(b"DATA")?;
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
