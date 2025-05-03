// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiFltAnimationNode {
    pub base: NiSwitchNode,
    pub period: f32,
}

impl Load for NiFltAnimationNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let period = stream.load()?;
        Ok(Self { base, period })
    }
}

impl Save for NiFltAnimationNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.period)?;
        Ok(())
    }
}

impl NiFltAnimationNode {
    flag_props! {
        bounce @ (mask = 0x0040) -> bool,
    }
}
