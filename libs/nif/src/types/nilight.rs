// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiLight {
    pub base: NiDynamicEffect,
    pub dimmer: f32,
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
}

impl Load for NiLight {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let dimmer = stream.load()?;
        let ambient_color = stream.load()?;
        let diffuse_color = stream.load()?;
        let specular_color = stream.load()?;
        Ok(Self {
            base,
            dimmer,
            ambient_color,
            diffuse_color,
            specular_color,
        })
    }
}

impl Save for NiLight {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.dimmer)?;
        stream.save(&self.ambient_color)?;
        stream.save(&self.diffuse_color)?;
        stream.save(&self.specular_color)?;
        Ok(())
    }
}
