// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiPlanarCollider {
    pub base: NiParticleCollider,
    pub height: f32,
    pub width: f32,
    pub position: Vec3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub normal: Vec3,
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
