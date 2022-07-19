// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleModifier {
    pub base: NiObject,
    pub next: NiLink<NiParticleModifier>,
    pub controller: NiLink<NiParticleSystemController>,
}

impl Load for NiParticleModifier {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let next = stream.load()?;
        let controller = stream.load()?;
        Ok(Self { base, next, controller })
    }
}

impl Save for NiParticleModifier {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.next)?;
        stream.save(&self.controller)?;
        Ok(())
    }
}
