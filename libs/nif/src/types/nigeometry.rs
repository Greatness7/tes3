// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiGeometry {
    pub base: NiAVObject,
    pub geometry_data: NiLink<NiGeometryData>,
    pub skin_instance: NiLink<NiSkinInstance>,
}

impl Load for NiGeometry {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let geometry_data = stream.load()?;
        let skin_instance = stream.load()?;
        Ok(Self {
            base,
            geometry_data,
            skin_instance,
        })
    }
}

impl Save for NiGeometry {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.geometry_data)?;
        stream.save(&self.skin_instance)?;
        Ok(())
    }
}
