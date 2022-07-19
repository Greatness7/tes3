// external imports
use nalgebra::{Matrix3, Vector3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiBoxBV {
    pub base: NiObject,
    pub center: Vector3<f32>,
    #[default(Matrix3::identity())]
    pub axis: Matrix3<f32>,
    pub extents: Vector3<f32>,
}

impl Load for NiBoxBV {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let center = stream.load()?;
        let axis = stream.load()?;
        let extents = stream.load()?;
        Ok(Self {
            base,
            center,
            axis,
            extents,
        })
    }
}

impl Save for NiBoxBV {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.center)?;
        stream.save(&self.axis)?;
        stream.save(&self.extents)?;
        Ok(())
    }
}
