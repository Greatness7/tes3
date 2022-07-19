// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct NiAccumulator {
    pub base: NiObject,
}

impl Load for NiAccumulator {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiAccumulator {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}
