// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSphericalCollider {
    pub base: NiParticleCollider,
    pub radius: f32,
    pub position: Vec3,
}

impl Load for NiSphericalCollider {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let radius = stream.load()?;
        let position = stream.load()?;
        Ok(Self { base, radius, position })
    }
}

impl Save for NiSphericalCollider {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.radius)?;
        stream.save(&self.position)?;
        Ok(())
    }
}
