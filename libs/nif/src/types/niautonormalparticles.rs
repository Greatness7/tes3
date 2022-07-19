// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiAutoNormalParticles {
    pub base: NiParticles,
}

impl Load for NiAutoNormalParticles {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiAutoNormalParticles {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}
