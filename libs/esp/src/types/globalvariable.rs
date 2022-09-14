// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlobalVariable {
    pub flags: ObjectFlags,
    pub id: String,
    pub kind: Option<GlobalType>,
    pub value: Option<f32>,
}

impl Load for GlobalVariable {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

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

impl Save for GlobalVariable {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
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
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
