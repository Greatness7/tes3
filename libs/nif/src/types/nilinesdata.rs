// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiLinesData {
    pub base: NiGeometryData,
    pub vertex_connectivity_flags: Vec<u8>,
}

impl Load for NiLinesData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base: NiGeometryData = stream.load()?;
        let vertex_connectivity_flags = stream.load_vec(base.vertices.len())?;
        Ok(Self {
            base,
            vertex_connectivity_flags,
        })
    }
}

impl Save for NiLinesData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_vec(&self.vertex_connectivity_flags)?;
        Ok(())
    }
}
