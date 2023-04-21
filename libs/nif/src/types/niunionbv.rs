// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiUnionBV {
    pub bounding_volumes: Vec<NiBoundingVolume>,
}

impl Load for NiUnionBV {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let bounding_volumes = stream.load()?;
        Ok(Self { bounding_volumes })
    }
}

impl Save for NiUnionBV {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.bounding_volumes)?;
        Ok(())
    }
}
