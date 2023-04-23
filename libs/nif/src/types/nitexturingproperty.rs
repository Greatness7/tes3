// internal imports
use crate::prelude::*;

const BUMP_INDEX: u32 = 5;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiTexturingProperty {
    pub base: NiProperty,
    pub apply_mode: ApplyMode,
    pub texture_maps: Vec<Option<TextureMap>>,
}

impl Load for NiTexturingProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let apply_mode = stream.load()?;
        let num_texture_maps: u32 = stream.load()?;
        let texture_maps = (0..num_texture_maps).load(|i| {
            Ok({
                let has_map = stream.load::<u32>()? != 0;
                if !has_map {
                    None
                } else if i == BUMP_INDEX {
                    Some(TextureMap::BumpMap(stream.load()?))
                } else {
                    Some(TextureMap::Map(stream.load()?))
                }
            })
        })?;
        Ok(Self {
            base,
            apply_mode,
            texture_maps,
        })
    }
}

impl Save for NiTexturingProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.apply_mode)?;
        stream.save_as::<u32>(self.texture_maps.len())?;
        for slot in &self.texture_maps {
            stream.save_as::<u32>(slot.is_some())?;
            match &slot {
                None => continue,
                Some(TextureMap::Map(map)) => stream.save(map)?,
                Some(TextureMap::BumpMap(map)) => stream.save(map)?,
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum TextureMap {
    #[default]
    Map(Map),
    BumpMap(BumpMap),
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct Map {
    pub base: NiObject,
    pub texture: NiLink<NiSourceTexture>,
    pub clamp_mode: ClampMode,
    pub filter_mode: FilterMode,
    pub texture_index: usize,
    pub ps2_l: i16,
    #[default((-75))]
    pub ps2_k: i16,
    pub unknown_flag1: u8,
    pub unknown_flag2: u8,
}

impl Load for Map {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let texture = stream.load()?;
        let clamp_mode = stream.load()?;
        let filter_mode = stream.load()?;
        let texture_index = stream.load_as::<u32, usize>()?;
        let ps2_l = stream.load()?;
        let ps2_k = stream.load()?;
        let unknown_flag1 = stream.load()?;
        let unknown_flag2 = stream.load()?;
        Ok(Self {
            base,
            texture,
            clamp_mode,
            filter_mode,
            texture_index,
            ps2_l,
            ps2_k,
            unknown_flag1,
            unknown_flag2,
        })
    }
}

impl Save for Map {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.texture)?;
        stream.save(&self.clamp_mode)?;
        stream.save(&self.filter_mode)?;
        stream.save_as::<u32>(self.texture_index)?;
        stream.save(&self.ps2_l)?;
        stream.save(&self.ps2_k)?;
        stream.save(&self.unknown_flag1)?;
        stream.save(&self.unknown_flag2)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct BumpMap {
    pub base: Map,
    pub luma_scale: f32,
    pub luma_offset: f32,
    #[default(Mat2::IDENTITY)]
    pub displacement: Mat2,
}

impl Load for BumpMap {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let luma_scale = stream.load()?;
        let luma_offset = stream.load()?;
        let displacement = stream.load()?;
        Ok(Self {
            base,
            luma_scale,
            luma_offset,
            displacement,
        })
    }
}

impl Save for BumpMap {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.luma_scale)?;
        stream.save(&self.luma_offset)?;
        stream.save(&self.displacement)?;
        Ok(())
    }
}
