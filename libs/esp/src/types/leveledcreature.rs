// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LeveledCreature {
    pub flags: ObjectFlags,
    pub id: String,
    pub list_flags: Option<u32>,
    pub chance_none: Option<u8>,
    pub creatures: Option<Vec<(String, u16)>>,
}

impl Load for LeveledCreature {
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
                    this.list_flags = Some(stream.load()?);
                }
                b"NNAM" => {
                    stream.expect(1u32)?;
                    this.chance_none = Some(stream.load()?);
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.creatures
                        .get_or_insert_with(default)
                        .reserve(stream.load_as::<u32, _>()?);
                }
                b"CNAM" => {
                    this.creatures.get_or_insert_with(default).push(default());
                    this.creatures.get_or_insert_with(default).last_mut().ok_or_else(err)?.0 = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(2u32)?;
                    this.creatures.get_or_insert_with(default).last_mut().ok_or_else(err)?.1 = stream.load()?;
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

impl Save for LeveledCreature {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
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
        if let Some(values) = self.creatures.as_ref().filter(|x| !x.is_empty()) {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save_as::<_, u32>(values.len())?;
            //
            for (item, level) in values {
                // CNAM
                stream.save(b"CNAM")?;
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
        "PC Level provided without a corresponding creature id",
    )
}
