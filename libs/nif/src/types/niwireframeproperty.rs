// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiWireframeProperty {
    pub base: NiProperty,
}

impl Load for NiWireframeProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiWireframeProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}

impl NiWireframeProperty {
    flag_props! {
        wireframe @ (mask = 0x0001) -> bool,
    }
}
