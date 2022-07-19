// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiUnionBV {
    pub base: NiObject,
    pub bounding_volumes: Vec<NiBoundingVolume>,
}

impl Load for NiUnionBV {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let bounding_volumes = stream.load()?;
        Ok(Self { base, bounding_volumes })
    }
}

impl Save for NiUnionBV {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.bounding_volumes)?;
        Ok(())
    }
}
