// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LeveledItem {
    pub flags: ObjectFlags,
    pub id: String,
    pub list_flags: u32,
    pub chance_none: u8,
    pub items: Vec<(String, u16)>,
}

impl Load for LeveledItem {
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
                    this.list_flags = stream.load()?;
                }
                b"NNAM" => {
                    stream.expect(1u32)?;
                    this.chance_none = stream.load()?;
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.items.reserve(stream.load_as::<u32, usize>()?);
                }
                b"INAM" => {
                    this.items.push(default());
                    this.items.last_mut().ok_or_else(err)?.0 = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(2u32)?;
                    this.items.last_mut().ok_or_else(err)?.1 = stream.load()?;
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

impl Save for LeveledItem {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.list_flags)?;
        // NNAM
        stream.save(b"NNAM")?;
        stream.save(&1u32)?;
        stream.save(&self.chance_none)?;
        // INDX
        if !self.items.is_empty() {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save_as::<u32>(self.items.len())?;
            //
            for (item, level) in &self.items {
                // INAM
                stream.save(b"INAM")?;
                stream.save(item)?;
                // INTV
                stream.save(b"INTV")?;
                stream.save(&2u32)?;
                stream.save(level)?;
            }
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

fn err() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "PC Level provided without a corresponding item id",
    )
}
