// external imports
use nalgebra::{Dynamic, OMatrix, Vector3, U2};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiLODNode {
    pub base: NiSwitchNode,
    pub lod_center: Vector3<f32>,
    #[default(OMatrix::<f32, U2, Dynamic>::zeros(0))]
    pub lod_levels: OMatrix<f32, U2, Dynamic>,
}

impl Load for NiLODNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let lod_center = stream.load()?;
        let num_lod_levels = stream.load_as::<u32, _>()?;
        let lod_levels = stream.load_matrix(2, num_lod_levels)?;
        Ok(Self {
            base,
            lod_center,
            lod_levels,
        })
    }
}

impl Save for NiLODNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.lod_center)?;
        stream.save_as::<_, u32>(self.lod_levels.ncols())?;
        stream.save_matrix(&self.lod_levels)?;
        Ok(())
    }
}
