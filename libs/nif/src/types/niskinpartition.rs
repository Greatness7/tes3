// external imports
use nalgebra::{dmatrix, dvector, Dynamic, OMatrix, OVector, U3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSkinPartition {
    pub base: NiObject,
    pub partitions: Vec<Partition>,
}

impl Load for NiSkinPartition {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let partitions = stream.load()?;
        Ok(Self { base, partitions })
    }
}

impl Save for NiSkinPartition {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.partitions)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct Partition {
    pub base: NiObject,
    #[default(dvector![])]
    pub bones: OVector<u16, Dynamic>,
    #[default(dvector![])]
    pub vertex_map: OVector<u16, Dynamic>,
    #[default(dmatrix![])]
    pub weights: OMatrix<f32, Dynamic, Dynamic>,
    #[default(OMatrix::<u16, U3, Dynamic>::zeros(0))]
    pub triangles: OMatrix<u16, U3, Dynamic>,
    #[default(dvector![])]
    pub strip_lengths: OVector<u16, Dynamic>,
    #[default(dvector![])]
    pub strips: OVector<u16, Dynamic>,
    pub bone_palette: Option<OMatrix<u8, Dynamic, Dynamic>>,
}

impl Load for Partition {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_vertices = stream.load_as::<u16, _>()?;
        let num_triangles = stream.load_as::<u16, _>()?;
        let num_bones = stream.load_as::<u16, _>()?;
        let num_strip_lengths = stream.load_as::<u16, _>()?;
        let num_bones_per_vertex = stream.load_as::<u16, _>()?;
        let bones = stream.load_matrix(num_bones, 1)?;
        let vertex_map = stream.load_matrix(num_vertices, 1)?;
        let weights = stream.load_matrix(num_vertices, num_bones_per_vertex)?;
        let triangles = stream.load_matrix(3, num_triangles)?;
        let strip_lengths = stream.load_matrix(num_strip_lengths, 1)?;
        let strip_lengths_sum = OVector::sum(&strip_lengths) as usize;
        let strips = stream.load_matrix(strip_lengths_sum, 1)?;
        let has_bone_palette: u8 = stream.load()?;
        let bone_palette = match has_bone_palette {
            0 => None,
            _ => Some(stream.load_matrix(num_vertices, num_bones_per_vertex)?),
        };
        Ok(Self {
            base,
            bones,
            vertex_map,
            weights,
            triangles,
            strip_lengths,
            strips,
            bone_palette,
        })
    }
}

impl Save for Partition {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u16>(self.vertex_map.len())?;
        stream.save_as::<_, u16>(self.triangles.ncols())?;
        stream.save_as::<_, u16>(self.bones.len())?;
        stream.save_as::<_, u16>(self.strip_lengths.len())?;
        stream.save_as::<_, u16>(self.weights.ncols())?;
        stream.save_matrix(&self.bones)?;
        stream.save_matrix(&self.vertex_map)?;
        stream.save_matrix(&self.weights)?;
        stream.save_matrix(&self.triangles)?;
        stream.save_matrix(&self.strip_lengths)?;
        stream.save_matrix(&self.strips)?;
        stream.save_as::<_, u8>(self.bone_palette.is_some())?;
        if let Some(bone_palette) = &self.bone_palette {
            stream.save_matrix(bone_palette)?;
        }
        Ok(())
    }
}
