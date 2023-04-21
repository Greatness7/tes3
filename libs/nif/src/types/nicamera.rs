// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiCamera {
    pub base: NiAVObject,
    pub view_frustum: [f32; 6], // NiFrustum
    pub view_port: [f32; 4],    // NiRect
    pub lod_adjust: f32,
    pub scene: NiLink<NiNode>,
    pub screen_polygons: Vec<NiLink<NiScreenPolygon>>,
}

impl Load for NiCamera {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let view_frustum = stream.load()?;
        let view_port = stream.load()?;
        let lod_adjust = stream.load()?;
        let scene = stream.load()?;
        let screen_polygons = stream.load()?;
        Ok(Self {
            base,
            view_frustum,
            view_port,
            lod_adjust,
            scene,
            screen_polygons,
        })
    }
}

impl Save for NiCamera {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.view_frustum)?;
        stream.save(&self.view_port)?;
        stream.save(&self.lod_adjust)?;
        stream.save(&self.scene)?;
        stream.save(&self.screen_polygons)?;
        Ok(())
    }
}
