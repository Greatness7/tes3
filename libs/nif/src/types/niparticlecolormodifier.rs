// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleColorModifier {
    pub base: NiParticleModifier,
    pub color_data: NiLink<NiColorData>,
}

impl Load for NiParticleColorModifier {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let color_data = stream.load()?;
        Ok(Self { base, color_data })
    }
}

impl Save for NiParticleColorModifier {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.color_data)?;
        Ok(())
    }
}
