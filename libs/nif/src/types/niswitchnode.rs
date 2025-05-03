// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSwitchNode {
    pub base: NiNode,
    pub active_index: usize,
}

impl Load for NiSwitchNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let active_index = stream.load_as::<u32, usize>()?;
        Ok(Self { base, active_index })
    }
}

impl Save for NiSwitchNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u32>(self.active_index)?;
        Ok(())
    }
}

impl NiSwitchNode {
    flag_props! {
        update_only_active @ (mask = 0x0020) -> bool,
    }
}
