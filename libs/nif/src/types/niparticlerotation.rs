// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiParticleRotation {
    pub base: NiParticleModifier,
    pub random_initial_axis: bool,
    pub initial_axis: Vec3,
    pub rotation_speed: f32,
}

impl Load for NiParticleRotation {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let random_initial_axis = stream.load::<u8>()? != 0;
        let initial_axis = stream.load()?;
        let rotation_speed = stream.load()?;
        Ok(Self {
            base,
            random_initial_axis,
            initial_axis,
            rotation_speed,
        })
    }
}

impl Save for NiParticleRotation {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u8>(self.random_initial_axis)?;
        stream.save(&self.initial_axis)?;
        stream.save(&self.rotation_speed)?;
        Ok(())
    }
}
