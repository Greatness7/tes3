// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSphereBV {
    pub bound: NiBound,
}

impl Load for NiSphereBV {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let bound = stream.load()?;
        Ok(Self { bound })
    }
}

impl Save for NiSphereBV {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.bound)?;
        Ok(())
    }
}
