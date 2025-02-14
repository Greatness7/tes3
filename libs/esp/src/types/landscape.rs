// external imports
use bytemuck::zeroed_box;
use glam::{Vec3, Vec4};

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

impl Landscape {
    pub fn decode_vertex_normals(&self) -> Vec<Vec3> {
        let data = self.vertex_normals.data.as_flattened();

        let mut normals = vec![Vec3::ZERO; 65 * 65];

        for (normal, &[x, y, z]) in normals.iter_mut().zip(data) {
            normal.x = x as f32;
            normal.y = y as f32;
            normal.z = z as f32;
            *normal = normal.normalize();
        }

        normals
    }

    pub fn decode_vertex_colors(&self) -> Vec<Vec4> {
        let data = self.vertex_colors.data.as_flattened();

        let mut colors = vec![Vec4::ZERO; 65 * 65];

        for (color, &[r, g, b]) in colors.iter_mut().zip(data) {
            color.x = r as f32;
            color.y = g as f32;
            color.z = b as f32;
            *color /= 255.0;
        }

        colors
    }

    pub fn decode_vertex_heights(&self) -> Box<[[f32; 65]; 65]> {
        let mut offset = self.vertex_heights.offset;
        let mut heights: Box<[[f32; 65]; 65]> = zeroed_box();

        for y in 0..65 {
            for x in 0..65 {
                let height = self.vertex_heights.data[y][x];
                offset += height as f32;
                heights[y][x] = offset;
            }
            offset = heights[y][0];
        }

        heights.iter_mut().flatten().for_each(|z| *z *= 8.0);
        heights
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_world_vertices(&self) -> Vec<Vec3> {
        const CELL_SIZE: f32 = 8192.0;

        let mut vertices = self.calculate_vertices();

        let x = self.grid.0 as f32 * CELL_SIZE;
        let y = self.grid.1 as f32 * CELL_SIZE;

        for vertex in &mut vertices {
            vertex.x += x;
            vertex.y += y;
        }

        vertices
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_vertices(&self) -> Vec<Vec3> {
        let mut vertices = vec![Vec3::ZERO; 65 * 65];

        let heights = self.decode_vertex_heights();

        for y in 0..65 {
            for x in 0..65 {
                let vertex = &mut vertices[y * 65 + x];
                vertex.x = x as f32 * 128.0;
                vertex.y = y as f32 * 128.0;
                vertex.z = heights[y][x];
            }
        }

        vertices
    }

    #[allow(clippy::many_single_char_names)]
    pub fn calcuate_triangles(&self) -> Vec<[u16; 3]> {
        let mut triangles = vec![[0; 3]; 8192];

        let v = 65u16;
        let t = v - 1;

        for (pair, i) in triangles.chunks_exact_mut(2).zip(0..) {
            let y = i / t;
            let x = i % t;

            let a = v * y + x;
            let b = a + v;
            let c = a + 1;
            let d = b + 1;

            let m = (x ^ y) & 1;
            let n = 1 - m;

            pair[0] = [d * n + a * m, b * n + c * m, a * n + b * m];
            pair[1] = [c * n + b * m, d * n + c * m, a * n + d * m];
        }

        triangles
    }
}
