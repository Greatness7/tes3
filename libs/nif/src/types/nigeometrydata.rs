// external imports
use nalgebra::{Dynamic, OMatrix, U2, U3, U4};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiGeometryData {
    pub base: NiObject,
    #[default(OMatrix::<f32, U3, Dynamic>::zeros(0))]
    pub vertices: OMatrix<f32, U3, Dynamic>,
    #[default(OMatrix::<f32, U3, Dynamic>::zeros(0))]
    pub normals: OMatrix<f32, U3, Dynamic>,
    pub bound: NiBound,
    #[default(OMatrix::<f32, U4, Dynamic>::zeros(0))]
    pub vertex_colors: OMatrix<f32, U4, Dynamic>,
    pub uv_sets: Vec<OMatrix<f32, U2, Dynamic>>,
}

impl Load for NiGeometryData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices = stream.load_as::<u16, _>()?;
        let has_vertices = stream.load::<u32>()? != 0;
        let num_vertices = if has_vertices { num_vertices } else { 0 };
        let vertices = stream.load_matrix(3, num_vertices)?;
        let has_normals = stream.load::<u32>()? != 0;
        let num_normals = if has_normals { num_vertices } else { 0 };
        let normals = stream.load_matrix(3, num_normals)?;
        let bound = stream.load()?;
        let has_vertex_colors = stream.load::<u32>()? != 0;
        let num_vertex_colors = if has_vertex_colors { num_vertices } else { 0 };
        let vertex_colors = stream.load_matrix(4, num_vertex_colors)?;
        let num_uv_sets: u16 = stream.load()?;
        let has_uv_sets = stream.load::<u32>()? != 0;
        let num_uv_sets = if has_uv_sets { num_uv_sets } else { 0 };
        let uv_sets = (0..num_uv_sets).load(|_| stream.load_matrix(2, num_vertices))?;
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
        stream.save_as::<_, u16>(self.vertices.ncols())?;
        stream.save_as::<_, u32>(!self.vertices.is_empty())?;
        stream.save_matrix(&self.vertices)?;
        stream.save_as::<_, u32>(!self.normals.is_empty())?;
        stream.save_matrix(&self.normals)?;
        stream.save(&self.bound)?;
        stream.save_as::<_, u32>(!self.vertex_colors.is_empty())?;
        stream.save_matrix(&self.vertex_colors)?;
        stream.save_as::<_, u16>(self.uv_sets.len())?;
        stream.save_as::<_, u32>(!self.uv_sets.is_empty())?;
        for uv_set in &self.uv_sets {
            stream.save_matrix(uv_set)?;
        }
        Ok(())
    }
}
