// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiGeometryData {
    pub base: NiObject,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub bound: NiBound,
    pub vertex_colors: Vec<ColorA>,
    pub uv_sets: Vec<Vec<Vec2>>, // TODO: flatten
}

impl Load for NiGeometryData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices = stream.load_as::<u16, _>()?;
        let has_vertices = stream.load::<u32>()? != 0;
        let num_vertices = if has_vertices { num_vertices } else { 0 };
        let vertices = stream.load_vec(num_vertices)?;
        let has_normals = stream.load::<u32>()? != 0;
        let num_normals = if has_normals { num_vertices } else { 0 };
        let normals = stream.load_vec(num_normals)?;
        let bound = stream.load()?;
        let has_vertex_colors = stream.load::<u32>()? != 0;
        let num_vertex_colors = if has_vertex_colors { num_vertices } else { 0 };
        let vertex_colors = stream.load_vec(num_vertex_colors)?;
        let num_uv_sets: u16 = stream.load()?;
        let has_uv_sets = stream.load::<u32>()? != 0;
        let num_uv_sets = if has_uv_sets { num_uv_sets } else { 0 };
        let uv_sets = (0..num_uv_sets).load(|_| stream.load_vec(num_vertices))?;
        Ok(Self {
            base,
            vertices,
            normals,
            bound,
            vertex_colors,
            uv_sets,
        })
    }
}

impl Save for NiGeometryData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u16>(self.vertices.len())?;
        stream.save_as::<u32>(!self.vertices.is_empty())?;
        stream.save_vec(&self.vertices)?;
        stream.save_as::<u32>(!self.normals.is_empty())?;
        stream.save_vec(&self.normals)?;
        stream.save(&self.bound)?;
        stream.save_as::<u32>(!self.vertex_colors.is_empty())?;
        stream.save_vec(&self.vertex_colors)?;
        stream.save_as::<u16>(self.uv_sets.len())?;
        stream.save_as::<u32>(!self.uv_sets.is_empty())?;
        for uv_set in &self.uv_sets {
            stream.save_vec(uv_set)?;
        }
        Ok(())
    }
}
