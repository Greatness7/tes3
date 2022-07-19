// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiProperty {
    pub base: NiObjectNET,
    pub flags: u16,
}

impl Load for NiProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let flags = stream.load()?;
        Ok(Self { base, flags })
    }
}

impl Save for NiProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.flags)?;
        Ok(())
    }
}
