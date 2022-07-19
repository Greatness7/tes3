// external imports
use nalgebra::{Dynamic, OMatrix, U4};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiRotatingParticlesData {
    pub base: NiParticlesData,
    #[default(OMatrix::<f32, U4, Dynamic>::zeros(0))]
    pub rotations: OMatrix<f32, U4, Dynamic>,
}

impl Load for NiRotatingParticlesData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base: NiParticlesData = stream.load()?;
        let has_rotations = stream.load::<u32>()? != 0;
        let num_rotations = if has_rotations { base.vertices.ncols() } else { 0 };
        let rotations = stream.load_matrix(4, num_rotations)?;
        Ok(Self { base, rotations })
    }
}

impl Save for NiRotatingParticlesData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u32>(!self.rotations.is_empty())?;
        stream.save_matrix(&self.rotations)?;
        Ok(())
    }
}
