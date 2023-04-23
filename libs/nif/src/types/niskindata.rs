// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiSkinData {
    pub base: NiObject,
    #[default(Mat3::IDENTITY)]
    pub rotation: Mat3,
    pub translation: Vec3,
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
        let bone_data = stream.load_seq(num_bone_data)?;
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
        stream.save_as::<u32>(self.bone_data.len())?;
        stream.save(&self.skin_partition)?;
        stream.save_seq(&self.bone_data)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct BoneData {
    #[default(Mat3::IDENTITY)]
    pub rotation: Mat3,
    pub translation: Vec3,
    #[default(1.0)]
    pub scale: f32,
    pub bound: NiBound,
    pub vertex_weights: Vec<(u16, f32)>,
}

impl Load for BoneData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let rotation = stream.load()?;
        let translation = stream.load()?;
        let scale = stream.load()?;
        let bound = stream.load()?;
        let num_vertex_weights: u16 = stream.load()?;
        let vertex_weights = stream.load_seq(num_vertex_weights)?;
        Ok(Self {
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
        stream.save(&self.rotation)?;
        stream.save(&self.translation)?;
        stream.save(&self.scale)?;
        stream.save(&self.bound)?;
        stream.save_as::<u16>(self.vertex_weights.len())?;
        stream.save_seq(&self.vertex_weights)?;
        Ok(())
    }
}
