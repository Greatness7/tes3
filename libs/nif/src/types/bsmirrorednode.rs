// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct BSMirroredNode {
    pub base: NiNode,
}

impl Load for BSMirroredNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for BSMirroredNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}
