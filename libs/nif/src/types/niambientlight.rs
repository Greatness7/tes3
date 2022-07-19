// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiAmbientLight {
    pub base: NiLight,
}

impl Load for NiAmbientLight {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiAmbientLight {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}
