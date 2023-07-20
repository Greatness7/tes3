// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LeveledCreature {
    pub flags: ObjectFlags,
    pub id: String,
    pub leveled_creature_flags: LeveledCreatureFlags,
    pub chance_none: u8,
    pub creatures: Vec<(String, u16)>,
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
                    this.leveled_creature_flags = stream.load()?;
                }
                b"NNAM" => {
                    stream.expect(1u32)?;
                    this.chance_none = stream.load()?;
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.creatures.reserve(stream.load_as::<u32, usize>()?);
                }
                b"CNAM" => {
                    this.creatures.push(default());
                    this.creatures.last_mut().ok_or_else(err)?.0 = stream.load()?;
                }
                b"INTV" => {
                    stream.expect(2u32)?;
                    this.creatures.last_mut().ok_or_else(err)?.1 = stream.load()?;
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
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.leveled_creature_flags)?;
        // NNAM
        stream.save(b"NNAM")?;
        stream.save(&1u32)?;
        stream.save(&self.chance_none)?;
        // INDX
        if !self.creatures.is_empty() {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save_as::<u32>(self.creatures.len())?;
            //
            for (item, level) in &self.creatures {
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
