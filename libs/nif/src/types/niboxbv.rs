// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiBoxBV {
    pub center: Vec3,
    #[default(Mat3::IDENTITY)]
    pub axis: Mat3,
    pub extents: Vec3,
}

impl Load for NiBoxBV {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let center = stream.load()?;
        let axis = stream.load()?;
        let extents = stream.load()?;
        Ok(Self { center, axis, extents })
    }
}

impl Save for NiBoxBV {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.center)?;
        stream.save(&self.axis)?;
        stream.save(&self.extents)?;
        Ok(())
    }
}
