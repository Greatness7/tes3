// external imports
use nalgebra::Vector3;

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiPlanarCollider {
    pub base: NiParticleCollider,
    pub height: f32,
    pub width: f32,
    pub position: Vector3<f32>,
    pub x_axis: Vector3<f32>,
    pub y_axis: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub constant: f32,
}

impl Load for NiPlanarCollider {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let height = stream.load()?;
        let width = stream.load()?;
        let position = stream.load()?;
        let x_axis = stream.load()?;
        let y_axis = stream.load()?;
        let normal = stream.load()?;
        let constant = stream.load()?;
        Ok(Self {
            base,
            height,
            width,
            position,
            x_axis,
            y_axis,
            normal,
            constant,
        })
    }
}

impl Save for NiPlanarCollider {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.height)?;
        stream.save(&self.width)?;
        stream.save(&self.position)?;
        stream.save(&self.x_axis)?;
        stream.save(&self.y_axis)?;
        stream.save(&self.normal)?;
        stream.save(&self.constant)?;
        Ok(())
    }
}
