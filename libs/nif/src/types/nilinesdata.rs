// external imports
use nalgebra::{dvector, Dynamic, OVector};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiLinesData {
    pub base: NiGeometryData,
    #[default(dvector![])]
    pub vertex_connectivity_flags: OVector<u8, Dynamic>,
}

impl Load for NiLinesData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base: NiGeometryData = stream.load()?;
        let vertex_connectivity_flags = stream.load_matrix(base.vertices.ncols(), 1)?;
        Ok(Self {
            base,
            vertex_connectivity_flags,
        })
    }
}

impl Save for NiLinesData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_matrix(&self.vertex_connectivity_flags)?;
        Ok(())
    }
}
