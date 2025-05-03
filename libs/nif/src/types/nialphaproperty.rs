// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiAlphaProperty {
    pub base: NiProperty,
    pub test_ref: u8,
}

impl Load for NiAlphaProperty {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let test_ref = stream.load()?;
        Ok(Self { base, test_ref })
    }
}

impl Save for NiAlphaProperty {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.test_ref)?;
        Ok(())
    }
}

impl NiAlphaProperty {
    flag_props! {
        alpha_blending @ (mask = 0x0001) -> bool,
        src_blend_mode @ (mask = 0x001E, pos = 1) -> AlphaBlendFunction,
        dst_blend_mode @ (mask = 0x01E0, pos = 5) -> AlphaBlendFunction,
        alpha_testing @ (mask = 0x0200) -> bool,
        test_mode @ (mask = 0x1C00, pos = 10) -> AlphaTestFunction,
    }
}
