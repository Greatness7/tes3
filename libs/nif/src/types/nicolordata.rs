// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiColorData {
    pub base: NiObject,
    pub keys: NiColorKey,
}

impl Load for NiColorData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let keys = stream.load()?;
        Ok(Self { base, keys })
    }
}

impl Save for NiColorData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.keys)?;
        Ok(())
    }
}
