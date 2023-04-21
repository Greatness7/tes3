// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiGravity {
    pub base: NiParticleModifier,
    pub decay: f32,
    pub strength: f32,
    pub force_type: ForceType,
    pub position: Vec3,
    pub direction: Vec3,
}

impl Load for NiGravity {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let decay = stream.load()?;
        let strength = stream.load()?;
        let force_type = stream.load()?;
        let position = stream.load()?;
        let direction = stream.load()?;
        Ok(Self {
            base,
            decay,
            strength,
            force_type,
            position,
            direction,
        })
    }
}

impl Save for NiGravity {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.decay)?;
        stream.save(&self.strength)?;
        stream.save(&self.force_type)?;
        stream.save(&self.position)?;
        stream.save(&self.direction)?;
        Ok(())
    }
}
