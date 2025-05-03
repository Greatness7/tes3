// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiBSPArrayController {
    pub base: NiParticleSystemController,
}

impl Load for NiBSPArrayController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiBSPArrayController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}

impl NiBSPArrayController {
    flag_props! {
        at_vertices @ (mask = 0x0010) -> bool,
    }
}
