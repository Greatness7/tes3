// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiZBufferProperty {
    pub base: NiProperty,
}

impl Load for NiZBufferProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        Ok(Self { base })
    }
}

impl Save for NiZBufferProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        Ok(())
    }
}

impl NiZBufferProperty {
    flag_props! {
        z_buffer_test @ (mask = 0x0001) -> bool,
        z_buffer_write @ (mask = 0x0002) -> bool,
        test_function @ (mask = 0x003C, pos = 2) -> ZBufferTestFunction,
    }
}
