// external imports
use nalgebra::{dvector, Dyn, OVector};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiVertWeightsExtraData {
    pub base: NiExtraData,
    #[default(dvector![])]
    pub weights: OVector<f32, Dyn>,
}

impl Load for NiVertWeightsExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_weights = stream.load_as::<u16, _>()?;
        let weights = stream.load_matrix(num_weights, 1)?;
        Ok(Self { base, weights })
    }
}

impl Save for NiVertWeightsExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u16>(self.weights.len())?;
        stream.save_matrix(&self.weights)?;
        Ok(())
    }
}
