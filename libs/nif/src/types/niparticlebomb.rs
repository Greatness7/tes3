// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleBomb {
    pub base: NiParticleModifier,
    pub decay: f32,
    pub duration: f32,
    pub delta_v: f32,
    pub start_time: f32,
    pub decay_type: DecayType,
    pub symmetry_type: SymmetryType,
    pub position: Vec3,
    pub direction: Vec3,
}

impl Load for NiParticleBomb {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let decay = stream.load()?;
        let duration = stream.load()?;
        let delta_v = stream.load()?;
        let start_time = stream.load()?;
        let decay_type = stream.load()?;
        let symmetry_type = stream.load()?;
        let position = stream.load()?;
        let direction = stream.load()?;
        Ok(Self {
            base,
            decay,
            duration,
            delta_v,
            start_time,
            decay_type,
            symmetry_type,
            position,
            direction,
        })
    }
}

impl Save for NiParticleBomb {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.decay)?;
        stream.save(&self.duration)?;
        stream.save(&self.delta_v)?;
        stream.save(&self.start_time)?;
        stream.save(&self.decay_type)?;
        stream.save(&self.symmetry_type)?;
        stream.save(&self.position)?;
        stream.save(&self.direction)?;
        Ok(())
    }
}
