// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct BipedObject {
    pub kind: BipedObjectType,
    pub male_bodypart: Option<String>,
    pub female_bodypart: Option<String>,
}

impl Load for BipedObject {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self::default();
        // INDX
        stream.expect(1u32)?;
        this.kind = stream.load()?;
        //
        for _ in 0..2 {
            // BNAM
            if stream.expect(*b"BNAM").is_ok() {
                this.male_bodypart = Some(stream.load()?);
                continue;
            }
            // CNAM
            if stream.expect(*b"CNAM").is_ok() {
                this.female_bodypart = Some(stream.load()?);
                continue;
            }
        }
        Ok(this)
    }
}

impl Save for BipedObject {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // INDX
        stream.save(b"INDX")?;
        stream.save(&1u32)?;
        stream.save(&self.kind)?;
        // BNAM
        if let Some(value) = &self.male_bodypart {
            stream.save(b"BNAM")?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.female_bodypart {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        Ok(())
    }
}
