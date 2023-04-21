// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiBSPNode {
    pub base: NiNode,
    pub plane: [f32; 4], // NiPlane
}

impl Load for NiBSPNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let plane = stream.load()?;
        Ok(Self { base, plane })
    }
}

impl Save for NiBSPNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.plane)?;
        Ok(())
    }
}
