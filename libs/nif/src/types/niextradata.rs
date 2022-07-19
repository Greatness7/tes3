// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiExtraData {
    pub base: NiObject,
    pub next: NiLink<NiExtraData>,
    pub bytes_remaining: u32,
}

impl Load for NiExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let next = stream.load()?;
        let bytes_remaining = stream.load()?;
        Ok(Self {
            base,
            next,
            bytes_remaining,
        })
    }
}

impl Save for NiExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.next)?;
        stream.save(&self.bytes_remaining)?;
        Ok(())
    }
}
