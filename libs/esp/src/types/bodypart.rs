// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Bodypart {
    pub flags: ObjectFlags,
    pub id: String,
    pub race: String,
    pub mesh: String,
    pub data: BodypartData,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BodypartData {
    pub part: BodypartId,
    pub vampire: bool,
    pub flags: BodypartFlags,
    pub bodypart_type: BodypartType,
}

impl Load for Bodypart {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = stream.load()?;
                }
                b"FNAM" => {
                    this.race = stream.load()?;
                }
                b"BYDT" => {
                    stream.expect(4u32)?;
                    this.data = stream.load()?;
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

impl Save for Bodypart {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if !self.mesh.is_empty() {
            stream.save(b"MODL")?;
            stream.save(&self.mesh)?;
        }
        // FNAM
        if !self.race.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.race)?;
        }
        // BYDT
        stream.save(b"BYDT")?;
        stream.save(&4u32)?;
        stream.save(&self.data)?;
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

impl Load for BodypartData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let part = stream.load()?;
        let vampire = stream.load::<u8>()? != 0;
        let flags = stream.load()?;
        let bodypart_type = stream.load()?;
        Ok(Self {
            part,
            vampire,
            flags,
            bodypart_type,
        })
    }
}

impl Save for BodypartData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.part)?;
        stream.save_as::<u8>(self.vampire)?;
        stream.save(&self.flags)?;
        stream.save(&self.bodypart_type)?;
        Ok(())
    }
}
