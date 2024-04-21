// external imports
use bytemuck::zeroed_box;

// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Landscape {
    pub flags: ObjectFlags,
    pub grid: (i32, i32),
    pub landscape_flags: LandscapeFlags,
    pub vertex_normals: VertexNormals,
    pub vertex_heights: VertexHeights,
    pub world_map_data: WorldMapData,
    pub vertex_colors: VertexColors,
    pub texture_indices: TextureIndices,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct VertexNormals {
    #[default(zeroed_box())]
    pub data: Box<[[[i8; 3]; 65]; 65]>,
}

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub struct VertexHeights {
    pub offset: f32,
    #[default(zeroed_box())]
    pub data: Box<[[i8; 65]; 65]>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct WorldMapData {
    #[default(zeroed_box())]
    pub data: Box<[[i8; 9]; 9]>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct VertexColors {
    #[default(zeroed_box())]
    pub data: Box<[[[u8; 3]; 65]; 65]>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct TextureIndices {
    #[default(zeroed_box())]
    pub data: Box<[[u16; 16]; 16]>,
}

impl Load for Landscape {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INTV" => {
                    stream.expect(8u32)?;
                    this.grid = stream.load()?;
                }
                b"DATA" => {
                    stream.expect(4u32)?;
                    this.landscape_flags = stream.load()?;
                }
                b"VNML" => {
                    stream.expect(12675u32)?;
                    this.vertex_normals = stream.load()?;
                }
                b"VHGT" => {
                    stream.expect(4232u32)?;
                    this.vertex_heights = stream.load()?;
                }
                b"WNAM" => {
                    stream.expect(81u32)?;
                    this.world_map_data = stream.load()?;
                }
                b"VCLR" => {
                    stream.expect(12675u32)?;
                    this.vertex_colors = stream.load()?;
                }
                b"VTEX" => {
                    stream.expect(512u32)?;
                    this.texture_indices = stream.load()?;
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::DELETED);
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
        stream.save(&self.flags)?;
        // INTV
        stream.save(b"INTV")?;
        stream.save(&8u32)?;
        stream.save(&self.grid)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&4u32)?;
        stream.save(&self.landscape_flags)?;
        //
        if self
            .landscape_flags
            .intersects(LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS)
        {
            // VNML
            stream.save(b"VNML")?;
            stream.save(&12675u32)?;
            stream.save(&self.vertex_normals)?;
            // VHGT
            stream.save(b"VHGT")?;
            stream.save(&4232u32)?;
            stream.save(&self.vertex_heights)?;
        }
        // WNAM
        if self.landscape_flags.uses_world_map_data() {
            stream.save(b"WNAM")?;
            stream.save(&81u32)?;
            stream.save(&self.world_map_data)?;
        }
        // VCLR
        if self.landscape_flags.intersects(LandscapeFlags::USES_VERTEX_COLORS) {
            stream.save(b"VCLR")?;
            stream.save(&12675u32)?;
            stream.save(&self.vertex_colors)?;
        }
        // VTEX
        if self.landscape_flags.intersects(LandscapeFlags::USES_TEXTURES) {
            stream.save(b"VTEX")?;
            stream.save(&512u32)?;
            stream.save(&self.texture_indices)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
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

impl LandscapeFlags {
    pub fn uses_world_map_data(&self) -> bool {
        self.intersects(Self::USES_VERTEX_HEIGHTS_AND_NORMALS | Self::USES_VERTEX_COLORS | Self::USES_TEXTURES)
    }
}
