// external imports
use nalgebra::{Dyn, OMatrix, U2, U3, U4};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiScreenPolygon {
    pub base: NiObject,
    #[default(Empty::empty())]
    pub vertices: OMatrix<f32, U3, Dyn>,
    #[default(Empty::empty())]
    pub uv_coords: OMatrix<f32, U2, Dyn>,
    #[default(Empty::empty())]
    pub vertex_colors: OMatrix<f32, U4, Dyn>,
    pub property_states: Vec<i32>,
}

impl Load for NiScreenPolygon {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices = stream.load_as::<u16, _>()?;
        let vertices = stream.load_matrix(3, num_vertices)?;
        let has_uv_coords = stream.load::<u32>()? != 0;
        let num_uv_coords = if has_uv_coords { num_vertices } else { 0 };
        let uv_coords = stream.load_matrix(2, num_uv_coords)?;
        let has_vertex_colors = stream.load::<u32>()? != 0;
        let num_vertex_colors = if has_vertex_colors { num_vertices } else { 0 };
        let vertex_colors = stream.load_matrix(4, num_vertex_colors)?;
        let property_states = stream.load()?;
        Ok(Self {
            base,
            vertices,
            uv_coords,
            vertex_colors,
            property_states,
        })
    }
}

impl Save for NiScreenPolygon {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u16>(self.vertices.ncols())?;
        stream.save_matrix(&self.vertices)?;
        stream.save_as::<_, u32>(!self.uv_coords.is_empty())?;
        stream.save_matrix(&self.uv_coords)?;
        stream.save_as::<_, u32>(!self.vertex_colors.is_empty())?;
        stream.save_matrix(&self.vertex_colors)?;
        stream.save(&self.property_states)?;
        Ok(())
    }
}
