// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiBSAnimationNode {
    pub base: NiNode,
}

impl Load for NiBSAnimationNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiBSAnimationNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}

impl NiBSAnimationNode {
    flag_props! {
        animated @ (mask = 0x0020) -> bool,
        not_random @ (mask = 0x0040) -> bool,
    }
}
