// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Landscape {
    pub flags1: u32,
    pub flags2: u32,
    pub grid: Option<(i32, i32)>,
    pub landscape_flags: Option<u32>,
    pub vertex_normals: Option<VertexNormals>,
    pub vertex_heights: Option<VertexHeights>,
    pub world_map_data: Option<WorldMapData>,
    pub vertex_colors: Option<VertexColors>,
    pub texture_indices: Option<TextureIndices>,
    pub deleted: Option<u32>,
}

#[derive(Meta, LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct VertexNormals {
    #[default(Box::new([0; 3 * 65 * 65]))]
    pub data: Box<[i8; 3 * 65 * 65]>,
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct VertexHeights {
    pub offset: f32,
    #[default(Box::new([0; 65 * 65]))]
    pub data: Box<[i8; 65 * 65]>,
}

#[derive(Meta, LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct WorldMapData {
    #[default(Box::new([0; 9 * 9]))]
    pub data: Box<[i8; 9 * 9]>,
}

#[derive(Meta, LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct VertexColors {
    #[default(Box::new([0; 3 * 65 * 65]))]
    pub data: Box<[u8; 3 * 65 * 65]>,
}

#[derive(Meta, LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct TextureIndices {
    #[default(Box::new([0; 16 * 16]))]
    pub data: Box<[u16; 16 * 16]>,
}

impl Load for Landscape {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INTV" => {
                    stream.expect(8u32)?;
                    this.grid = Some(stream.load()?);
                }
                b"DATA" => {
                    stream.expect(4u32)?;
                    this.landscape_flags = Some(stream.load()?);
                }
                b"VNML" => {
                    stream.expect(12675u32)?;
                    this.vertex_normals = Some(stream.load()?);
                }
                b"VHGT" => {
                    stream.expect(4232u32)?;
                    this.vertex_heights = Some(stream.load()?);
                }
                b"WNAM" => {
                    stream.expect(81u32)?;
                    this.world_map_data = Some(stream.load()?);
                }
                b"VCLR" => {
                    stream.expect(12675u32)?;
                    this.vertex_colors = Some(stream.load()?);
                }
                b"VTEX" => {
                    stream.expect(512u32)?;
                    this.texture_indices = Some(stream.load()?);
                }
                b"DELE" => {
                    stream.expect(4u32)?;
                    this.deleted = Some(stream.load()?);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Landscape {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // INTV
        if let Some(value) = &self.grid {
            stream.save(b"INTV")?;
            stream.save(&8u32)?;
            stream.save(value)?;
        }
        // DATA
        if let Some(value) = &self.landscape_flags {
            stream.save(b"DATA")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // VNML
        if let Some(value) = &self.vertex_normals {
            stream.save(b"VNML")?;
            stream.save(&12675u32)?;
            stream.save(value)?;
        }
        // VHGT
        if let Some(value) = &self.vertex_heights {
            stream.save(b"VHGT")?;
            stream.save(&4232u32)?;
            stream.save(value)?;
        }
        // WNAM
        if let Some(value) = &self.world_map_data {
            stream.save(b"WNAM")?;
            stream.save(&81u32)?;
            stream.save(value)?;
        }
        // VCLR
        if let Some(value) = &self.vertex_colors {
            stream.save(b"VCLR")?;
            stream.save(&12675u32)?;
            stream.save(value)?;
        }
        // VTEX
        if let Some(value) = &self.texture_indices {
            stream.save(b"VTEX")?;
            stream.save(&512u32)?;
            stream.save(value)?;
        }
        // DELE
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}

impl Load for VertexHeights {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let offset = stream.load()?;
        let data = stream.load()?;
        stream.skip(3)?; // padding
        Ok(Self { offset, data })
    }
}

impl Save for VertexHeights {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.offset)?;
        stream.save(&self.data)?;
        stream.save(&[0u8; 3])?; // padding
        Ok(())
    }
}