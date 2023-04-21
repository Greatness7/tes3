// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiVertWeightsExtraData {
    pub base: NiExtraData,
    pub weights: Vec<f32>,
}

impl Load for NiVertWeightsExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_weights: u16 = stream.load()?;
        let weights = stream.load_vec(num_weights)?;
        Ok(Self { base, weights })
    }
}

impl Save for NiVertWeightsExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u16>(self.weights.len())?;
        stream.save_vec(&self.weights)?;
        Ok(())
    }
}
