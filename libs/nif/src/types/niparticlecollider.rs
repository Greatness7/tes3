// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleCollider {
    pub base: NiParticleModifier,
    pub bounce: f32,
}

impl Load for NiParticleCollider {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let bounce = stream.load()?;
        Ok(Self { base, bounce })
    }
}

impl Save for NiParticleCollider {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.bounce)?;
        Ok(())
    }
}
