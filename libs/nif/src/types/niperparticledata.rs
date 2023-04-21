// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiPerParticleData {
    pub velocity: Vec3,
    pub rotation_axis: Vec3,
    pub age: f32,
    pub lifespan: f32,
    pub last_update: f32,
    pub generation: u16,
    pub index: u16,
}

impl Load for NiPerParticleData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let velocity = stream.load()?;
        let rotation_axis = stream.load()?;
        let age = stream.load()?;
        let lifespan = stream.load()?;
        let last_update = stream.load()?;
        let generation = stream.load()?;
        let index = stream.load()?;
        Ok(Self {
            velocity,
            rotation_axis,
            age,
            lifespan,
            last_update,
            generation,
            index,
        })
    }
}

impl Save for NiPerParticleData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.velocity)?;
        stream.save(&self.rotation_axis)?;
        stream.save(&self.age)?;
        stream.save(&self.lifespan)?;
        stream.save(&self.last_update)?;
        stream.save(&self.generation)?;
        stream.save(&self.index)?;
        Ok(())
    }
}
