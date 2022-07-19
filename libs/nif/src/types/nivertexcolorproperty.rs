// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiVertexColorProperty {
    pub base: NiProperty,
    pub source_vertex_mode: SourceVertexMode,
    pub lighting_mode: LightingMode,
}

impl Load for NiVertexColorProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let source_vertex_mode = stream.load()?;
        let lighting_mode = stream.load()?;
        Ok(Self {
            base,
            source_vertex_mode,
            lighting_mode,
        })
    }
}

impl Save for NiVertexColorProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.source_vertex_mode)?;
        stream.save(&self.lighting_mode)?;
        Ok(())
    }
}
