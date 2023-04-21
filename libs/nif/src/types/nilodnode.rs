// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiLODNode {
    pub base: NiSwitchNode,
    pub lod_center: Vec3,
    pub lod_levels: Vec<[f32; 2]>, // [Near, Far]
}

impl Load for NiLODNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let lod_center = stream.load()?;
        let num_lod_levels: u32 = stream.load()?;
        let lod_levels = stream.load_vec(num_lod_levels)?;
        Ok(Self {
            base,
            lod_center,
            lod_levels,
        })
    }
}

impl Save for NiLODNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.lod_center)?;
        stream.save_as::<u32>(self.lod_levels.len())?;
        stream.save_vec(&self.lod_levels)?;
        Ok(())
    }
}
