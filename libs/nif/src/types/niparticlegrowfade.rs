// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleGrowFade {
    pub base: NiParticleModifier,
    pub grow_time: f32,
    pub fade_time: f32,
}

impl Load for NiParticleGrowFade {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let grow_time = stream.load()?;
        let fade_time = stream.load()?;
        Ok(Self {
            base,
            grow_time,
            fade_time,
        })
    }
}

impl Save for NiParticleGrowFade {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.grow_time)?;
        stream.save(&self.fade_time)?;
        Ok(())
    }
}
