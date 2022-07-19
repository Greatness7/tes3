// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSpotLight {
    pub base: NiPointLight,
    pub outer_spot_angle: f32,
    pub exponent: f32,
}

impl Load for NiSpotLight {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let outer_spot_angle = stream.load()?;
        let exponent = stream.load()?;
        Ok(Self {
            base,
            outer_spot_angle,
            exponent,
        })
    }
}

impl Save for NiSpotLight {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.outer_spot_angle)?;
        stream.save(&self.exponent)?;
        Ok(())
    }
}
