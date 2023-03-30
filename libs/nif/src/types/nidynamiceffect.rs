// external imports
use nalgebra::{dvector, Dyn, OVector};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiDynamicEffect {
    pub base: NiAVObject,
    #[default(dvector![])]
    pub affected_nodes: OVector<i32, Dyn>, // Invalid Links
}

impl Load for NiDynamicEffect {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_affected_nodes = stream.load_as::<u32, _>()?;
        let affected_nodes = stream.load_matrix(num_affected_nodes, 1)?;
        Ok(Self { base, affected_nodes })
    }
}

impl Save for NiDynamicEffect {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u32>(self.affected_nodes.len())?;
        stream.save_matrix(&self.affected_nodes)?;
        Ok(())
    }
}
