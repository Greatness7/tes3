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

impl NiGeometry {
    flag_props! {
        compress_vertices @ (mask = 0x0008) -> bool,
        compress_normals @ (mask = 0x0010) -> bool,
        compress_uv_sets @ (mask = 0x0020) -> bool,
        shadow @ (mask = 0x0040) -> bool,
    }
}
