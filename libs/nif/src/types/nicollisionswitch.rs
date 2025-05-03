// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiCollisionSwitch {
    pub base: NiNode,
}

impl Load for NiCollisionSwitch {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiCollisionSwitch {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}

impl NiCollisionSwitch {
    flag_props! {
        propagate @ (mask = 0x0020) -> bool,
    }
}
