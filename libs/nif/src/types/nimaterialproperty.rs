// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiMaterialProperty {
    pub base: NiProperty,
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub emissive_color: Vec3,
    pub shine: f32,
    pub alpha: f32,
}

impl Load for NiMaterialProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let ambient_color = stream.load()?;
        let diffuse_color = stream.load()?;
        let specular_color = stream.load()?;
        let emissive_color = stream.load()?;
        let shine = stream.load()?;
        let alpha = stream.load()?;
        Ok(Self {
            base,
            ambient_color,
            diffuse_color,
            specular_color,
            emissive_color,
            shine,
            alpha,
        })
    }
}

impl Save for NiMaterialProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.ambient_color)?;
        stream.save(&self.diffuse_color)?;
        stream.save(&self.specular_color)?;
        stream.save(&self.emissive_color)?;
        stream.save(&self.shine)?;
        stream.save(&self.alpha)?;
        Ok(())
    }
}
