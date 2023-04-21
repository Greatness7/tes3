// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiFogProperty {
    pub base: NiProperty,
    pub fog_depth: f32,
    pub fog_color: Vec3,
}

impl Load for NiFogProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let fog_depth = stream.load()?;
        let fog_color = stream.load()?;
        Ok(Self {
            base,
            fog_depth,
            fog_color,
        })
    }
}

impl Save for NiFogProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.fog_depth)?;
        stream.save(&self.fog_color)?;
        Ok(())
    }
}
