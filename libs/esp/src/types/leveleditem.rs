// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct LeveledItem {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub list_flags: Option<u32>,
    pub chance_none: Option<u8>,
    pub items: Option<Vec<(String, u16)>>,
    pub deleted: Option<u32>,
}

impl Load for LeveledItem {
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
                    stream.expect(4u32)?;
                    this.list_flags = Some(stream.load()?);
                }
                b"NNAM" => {
                    stream.expect(1u32)?;
                    this.chance_none = Some(stream.load()?);
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    let len: u32 = stream.load()?;
                    this.items.get_or_insert_default().reserve(len as usize);
                }
                b"INAM" => {
                    this.items.get_or_insert_default().push(default());
                    this.items.get_or_insert_default().last_mut().ok_or_else(err)?.0 = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(2u32)?;
                    this.items.get_or_insert_default().last_mut().ok_or_else(err)?.1 = stream.load()?;
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

impl Save for LeveledItem {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // DATA
        if let Some(value) = &self.list_flags {
            stream.save(b"DATA")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // NNAM
        if let Some(value) = &self.chance_none {
            stream.save(b"NNAM")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        // INDX
        if let Some(values) = self.items.as_ref().filter(|x| !x.is_empty()) {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save_as::<_, u32>(values.len())?;
            //
            for (item, level) in values {
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
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
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
