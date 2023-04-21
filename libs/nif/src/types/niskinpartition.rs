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
    pub num_bones_per_vertex: u16,
    pub bones: Vec<u16>,
    pub vertex_indices: Vec<u16>,
    pub weights: Vec<f32>,
    pub triangles: Vec<[u16; 3]>,
    pub strip_lengths: Vec<u16>,
    pub strips: Vec<u16>,
    pub bone_palette: Option<Vec<u8>>,
}

impl Load for Partition {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_vertices: u16 = stream.load()?;
        let num_triangles: u16 = stream.load()?;
        let num_bones: u16 = stream.load()?;
        let num_strip_lengths: u16 = stream.load()?;
        let num_bones_per_vertex: u16 = stream.load()?;
        let num_weights = num_vertices * num_bones_per_vertex;
        let bones = stream.load_vec(num_bones)?;
        let vertex_indices = stream.load_vec(num_vertices)?;
        let weights = stream.load_vec(num_weights)?;
        let triangles = stream.load_vec(num_triangles)?;
        let strip_lengths: Vec<u16> = stream.load_vec(num_strip_lengths)?;
        let strip_lengths_sum: usize = strip_lengths.iter().map(|n| *n as usize).sum();
        let strips = stream.load_vec(strip_lengths_sum)?;
        let has_bone_palette: u8 = stream.load()?;
        let bone_palette = match has_bone_palette {
            0 => None,
            _ => Some(stream.load_vec(num_weights)?),
        };
        Ok(Self {
            num_bones_per_vertex,
            bones,
            vertex_indices,
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
        stream.save_as::<u16>(self.vertex_indices.len())?;
        stream.save_as::<u16>(self.triangles.len())?;
        stream.save_as::<u16>(self.bones.len())?;
        stream.save_as::<u16>(self.strip_lengths.len())?;
        stream.save_as::<u16>(self.num_bones_per_vertex)?;
        stream.save_vec(&self.bones)?;
        stream.save_vec(&self.vertex_indices)?;
        stream.save_vec(&self.weights)?;
        stream.save_vec(&self.triangles)?;
        stream.save_vec(&self.strip_lengths)?;
        stream.save_vec(&self.strips)?;
        stream.save_as::<u8>(self.bone_palette.is_some())?;
        if let Some(bone_palette) = &self.bone_palette {
            stream.save_vec(bone_palette)?;
        }
        Ok(())
    }
}
