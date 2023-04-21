// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiGeomMorpherController {
    pub base: NiMorpherController,
    pub always_update: bool,
}

impl Load for NiGeomMorpherController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let always_update = stream.load::<u8>()? != 0;
        Ok(Self { base, always_update })
    }
}

impl Save for NiGeomMorpherController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u8>(self.always_update)?;
        Ok(())
    }
}
