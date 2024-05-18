// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct StartScript {
    pub flags: ObjectFlags,
    pub id: String,
    pub script: Option<String>,
}

impl Load for StartScript {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"DATA" => {
                    this.id = stream.load()?;
                }
                b"NAME" => {
                    this.script = Some(stream.load()?);
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

impl Save for StartScript {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&self.id)?;
        // NAME
        if let Some(value) = &self.script {
            stream.save(b"NAME")?;
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
