// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Door {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub script: Option<String>,
    pub open_sound: Option<String>,
    pub close_sound: Option<String>,
}

impl Load for Door {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"SNAM" => {
                    this.open_sound = Some(stream.load()?);
                }
                b"ANAM" => {
                    this.close_sound = Some(stream.load()?);
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

impl Save for Door {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if let Some(value) = &self.mesh {
            stream.save(b"MODL")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // SNAM
        if let Some(value) = &self.open_sound {
            stream.save(b"SNAM")?;
            stream.save(value)?;
        }
        // ANAM
        if let Some(value) = &self.close_sound {
            stream.save(b"ANAM")?;
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
