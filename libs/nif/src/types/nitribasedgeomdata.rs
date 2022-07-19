// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiTriBasedGeomData {
    pub base: NiGeometryData,
}

impl Load for NiTriBasedGeomData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiTriBasedGeomData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}
