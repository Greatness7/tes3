// external imports
use nalgebra::Vector3;

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiBound {
    pub base: NiObject,
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Load for NiBound {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let center = stream.load()?;
        let radius = stream.load()?;
        Ok(Self { base, center, radius })
    }
}

impl Save for NiBound {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_matrix(&self.center)?;
        stream.save(&self.radius)?;
        Ok(())
    }
}
