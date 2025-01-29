// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiGeometryData {
    pub base: NiObject,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub bound: NiBound,
    pub vertex_colors: Vec<ColorA>,
    pub uv_sets: Vec<Vec2>,
}

impl Load for NiGeometryData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices = stream.load_as::<u16, usize>()?;
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
        let num_uv_sets = stream.load_as::<u16, usize>()?;
        let has_uv_sets = stream.load::<u32>()? != 0;
        let num_uv_sets = if has_uv_sets { num_uv_sets } else { 0 };
        let uv_sets = stream.load_vec(num_vertices * num_uv_sets)?;
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
        stream.save_as::<u16>(self.num_uv_sets())?;
        stream.save_as::<u32>(!self.uv_sets.is_empty())?;
        stream.save_vec(&self.uv_sets)?;
        Ok(())
    }
}

impl NiGeometryData {
    pub fn num_uv_sets(&self) -> usize {
        if self.vertices.is_empty() {
            0
        } else {
            self.uv_sets.len() / self.vertices.len()
        }
    }

    pub fn uv_set(&self, index: usize) -> Option<&[Vec2]> {
        let start = index * self.vertices.len();
        let end = start + self.vertices.len();
        self.uv_sets.get(start..end)
    }

    pub fn uv_set_mut(&mut self, index: usize) -> Option<&mut [Vec2]> {
        let start = index * self.vertices.len();
        let end = start + self.vertices.len();
        self.uv_sets.get_mut(start..end)
    }

    pub fn update_center_radius(&mut self) {
        if self.vertices.is_empty() {
            self.bound.center = Vec3::ZERO;
            self.bound.radius = 0.0;
            return;
        }

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for v in &self.vertices {
            min = min.min(*v);
            max = max.max(*v);
        }

        let center = 0.5 * (min + max);
        let mut radius = 0.0;

        for v in &self.vertices {
            let d = *v - center;
            radius = d.dot(d).max(radius);
        }

        self.bound.center = center;
        self.bound.radius = radius.sqrt();
    }
}
