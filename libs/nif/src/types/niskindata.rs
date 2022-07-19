// external imports
use nalgebra::{Matrix3, Vector3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiSkinData {
    pub base: NiObject,
    #[default(Matrix3::identity())]
    pub rotation: Matrix3<f32>,
    pub translation: Vector3<f32>,
    #[default(1.0)]
    pub scale: f32,
    pub skin_partition: NiLink<NiSkinPartition>,
    pub bone_data: Vec<BoneData>,
}

impl Load for NiSkinData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let rotation = stream.load()?;
        let translation = stream.load()?;
        let scale = stream.load()?;
        let num_bone_data: u32 = stream.load()?;
        let skin_partition = stream.load()?;
        let bone_data = (0..num_bone_data).load(|_| stream.load())?;
        Ok(Self {
            base,
            rotation,
            translation,
            scale,
            skin_partition,
            bone_data,
        })
    }
}

impl Save for NiSkinData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.rotation)?;
        stream.save(&self.translation)?;
        stream.save(&self.scale)?;
        stream.save_as::<_, u32>(self.bone_data.len())?;
        stream.save(&self.skin_partition)?;
        for bone_data in &self.bone_data {
            stream.save(bone_data)?;
        }
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct BoneData {
    pub base: NiObject,
    #[default(Matrix3::identity())]
    pub rotation: Matrix3<f32>,
    pub translation: Vector3<f32>,
    #[default(1.0)]
    pub scale: f32,
    pub bound: NiBound,
    pub vertex_weights: Vec<(u16, f32)>,
}

impl Load for BoneData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let rotation = stream.load()?;
        let translation = stream.load()?;
        let scale = stream.load()?;
        let bound = stream.load()?;
        let num_vertex_weights: u16 = stream.load()?;
        let vertex_weights = (0..num_vertex_weights).load(|_| Ok((stream.load()?, stream.load()?)))?;
        Ok(Self {
            base,
            rotation,
            translation,
            scale,
            bound,
            vertex_weights,
        })
    }
}

impl Save for BoneData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.rotation)?;
        stream.save(&self.translation)?;
        stream.save(&self.scale)?;
        stream.save(&self.bound)?;
        stream.save_as::<_, u16>(self.vertex_weights.len())?;
        for (index, weight) in &self.vertex_weights {
            stream.save(index)?;
            stream.save(weight)?;
        }
        Ok(())
    }
}
