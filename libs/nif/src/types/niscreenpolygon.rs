// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiScreenPolygon {
    pub base: NiObject,
    pub vertices: Vec<Vec3>,
    pub uv_coords: Vec<Vec2>,
    pub vertex_colors: Vec<ColorA>,
    pub property_states: Vec<i32>,
}

impl Load for NiScreenPolygon {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices: u16 = stream.load()?;
        let vertices = stream.load_vec(num_vertices)?;
        let has_uv_coords = stream.load::<u32>()? != 0;
        let num_uv_coords = if has_uv_coords { num_vertices } else { 0 };
        let uv_coords = stream.load_vec(num_uv_coords)?;
        let has_vertex_colors = stream.load::<u32>()? != 0;
        let num_vertex_colors = if has_vertex_colors { num_vertices } else { 0 };
        let vertex_colors = stream.load_vec(num_vertex_colors)?;
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
        stream.save_as::<u16>(self.vertices.len())?;
        stream.save_vec(&self.vertices)?;
        stream.save_as::<u32>(!self.uv_coords.is_empty())?;
        stream.save_vec(&self.uv_coords)?;
        stream.save_as::<u32>(!self.vertex_colors.is_empty())?;
        stream.save_vec(&self.vertex_colors)?;
        stream.save(&self.property_states)?;
        Ok(())
    }
}
