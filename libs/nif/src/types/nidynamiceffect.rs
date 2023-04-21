// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiDynamicEffect {
    pub base: NiAVObject,
    pub affected_nodes: Vec<i32>, // Invalid Links
}

impl Load for NiDynamicEffect {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_affected_nodes: u32 = stream.load()?;
        let affected_nodes = stream.load_vec(num_affected_nodes)?;
        Ok(Self { base, affected_nodes })
    }
}

impl Save for NiDynamicEffect {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u32>(self.affected_nodes.len())?;
        stream.save_vec(&self.affected_nodes)?;
        Ok(())
    }
}
