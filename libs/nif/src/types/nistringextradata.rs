// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiStringExtraData {
    pub base: NiExtraData,
    pub value: String,
}

impl Load for NiStringExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let value = stream.load()?;
        Ok(Self { base, value })
    }
}

impl Save for NiStringExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.value)?;
        Ok(())
    }
}
