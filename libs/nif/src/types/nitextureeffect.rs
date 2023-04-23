// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTextureEffect {
    pub base: NiDynamicEffect,
    #[default(Mat3::IDENTITY)]
    pub projection_matrix: Mat3,
    pub projection_translation: Vec3,
    pub texture_filter: FilterMode,
    pub texture_clamp: ClampMode,
    pub texture_type: TextureType,
    pub coordinate_generation_type: CoordGenType,
    pub source_texture: NiLink<NiSourceTexture>,
    pub clipping_plane_enable: u8,
    pub clipping_plane: [f32; 4], // NiPlane
    pub ps2_l: i16,
    #[default((-75))]
    pub ps2_k: i16,
    pub unknown_byte1: u8,
    pub unknown_byte2: u8,
}

impl Load for NiTextureEffect {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let projection_matrix = stream.load()?;
        let projection_translation = stream.load()?;
        let texture_filter = stream.load()?;
        let texture_clamp = stream.load()?;
        let texture_type = stream.load()?;
        let coordinate_generation_type = stream.load()?;
        let source_texture = stream.load()?;
        let clipping_plane_enable = stream.load()?;
        let clipping_plane = stream.load()?;
        let ps2_l = stream.load()?;
        let ps2_k = stream.load()?;
        let unknown_byte1 = stream.load()?;
        let unknown_byte2 = stream.load()?;
        Ok(Self {
            base,
            projection_matrix,
            projection_translation,
            texture_filter,
            texture_clamp,
            texture_type,
            coordinate_generation_type,
            source_texture,
            clipping_plane_enable,
            clipping_plane,
            ps2_l,
            ps2_k,
            unknown_byte1,
            unknown_byte2,
        })
    }
}

impl Save for NiTextureEffect {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.projection_matrix)?;
        stream.save(&self.projection_translation)?;
        stream.save(&self.texture_filter)?;
        stream.save(&self.texture_clamp)?;
        stream.save(&self.texture_type)?;
        stream.save(&self.coordinate_generation_type)?;
        stream.save(&self.source_texture)?;
        stream.save(&self.clipping_plane_enable)?;
        stream.save(&self.clipping_plane)?;
        stream.save(&self.ps2_l)?;
        stream.save(&self.ps2_k)?;
        stream.save(&self.unknown_byte1)?;
        stream.save(&self.unknown_byte2)?;
        Ok(())
    }
}
