// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Enchanting {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub data: Option<EnchantingData>,
    pub effects: Option<Vec<Effect>>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct EnchantingData {
    pub kind: EnchantType,
    pub cost: u32,
    pub max_charge: u32,
    pub flags: u32,
}

impl Load for Enchanting {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"ENDT" => {
                    stream.expect(16u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ENAM" => {
                    stream.expect(24u32)?;
                    this.effects.get_or_insert_with(default).push(stream.load()?);
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

impl Save for Enchanting {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // ENDT
        if let Some(value) = &self.data {
            stream.save(b"ENDT")?;
            stream.save(&16u32)?;
            stream.save(value)?;
        }
        // ENAM
        for effect in self.effects.iter().flatten() {
            stream.save(b"ENAM")?;
            stream.save(&24u32)?;
            stream.save(effect)?;
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
