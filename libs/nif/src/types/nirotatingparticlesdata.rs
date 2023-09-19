// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiRotatingParticlesData {
    pub base: NiParticlesData,
    pub rotations: Vec<Quat>,
}

impl Load for NiRotatingParticlesData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base: NiParticlesData = stream.load()?;
        let has_rotations = stream.load::<u32>()? != 0;
        let num_rotations = if has_rotations { base.vertices.len() } else { 0 };
        let rotations = stream.load_seq(num_rotations)?;
        Ok(Self { base, rotations })
    }
}

impl Save for NiRotatingParticlesData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u32>(!self.rotations.is_empty())?;
        stream.save_seq(&self.rotations)?;
        Ok(())
    }
}
