// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct AiData {
    pub hello: i16,
    pub fight: i8,
    pub flee: i8,
    pub alarm: i8,
    pub services: u32,
}

impl Load for AiData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let hello = stream.load()?;
        let fight = stream.load()?;
        let flee = stream.load()?;
        let alarm = stream.load()?;
        stream.skip(3)?; // padding
        let services = stream.load()?;
        Ok(Self {
            hello,
            fight,
            flee,
            alarm,
            services,
        })
    }
}

impl Save for AiData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.hello)?;
        stream.save(&self.fight)?;
        stream.save(&self.flee)?;
        stream.save(&self.alarm)?;
        stream.save(&[0u8; 3])?; // padding
        stream.save(&self.services)?;
        Ok(())
    }
}
