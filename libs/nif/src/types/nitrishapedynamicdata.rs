// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiTriShapeDynamicData {
    pub base: NiTriShapeData,
    pub active_vertices: u16,
    pub active_triangles: u16,
}

impl Load for NiTriShapeDynamicData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let active_vertices = stream.load()?;
        let active_triangles = stream.load()?;
        Ok(Self {
            base,
            active_vertices,
            active_triangles,
        })
    }
}

impl Save for NiTriShapeDynamicData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.active_vertices)?;
        stream.save(&self.active_triangles)?;
        Ok(())
    }
}
