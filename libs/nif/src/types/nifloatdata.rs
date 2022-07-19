// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiFloatData {
    pub base: NiObject,
    pub keys: NiFloatKey,
}

impl Load for NiFloatData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let keys = stream.load()?;
        Ok(Self { base, keys })
    }
}

impl Save for NiFloatData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.keys)?;
        Ok(())
    }
}
