// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiPointLight {
    pub base: NiLight,
    pub constant_attenuation: f32,
    pub linear_attenuation: f32,
    pub quadratic_attenuation: f32,
}

impl Load for NiPointLight {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let constant_attenuation = stream.load()?;
        let linear_attenuation = stream.load()?;
        let quadratic_attenuation = stream.load()?;
        Ok(Self {
            base,
            constant_attenuation,
            linear_attenuation,
            quadratic_attenuation,
        })
    }
}

impl Save for NiPointLight {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.constant_attenuation)?;
        stream.save(&self.linear_attenuation)?;
        stream.save(&self.quadratic_attenuation)?;
        Ok(())
    }
}
