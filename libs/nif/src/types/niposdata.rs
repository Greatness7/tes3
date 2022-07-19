// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiPosData {
    pub base: NiObject,
    pub keys: NiPosKey,
}

impl Load for NiPosData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let keys = stream.load()?;
        Ok(Self { base, keys })
    }
}

impl Save for NiPosData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.keys)?;
        Ok(())
    }
}
