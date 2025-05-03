// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiLookAtController {
    pub base: NiTimeController,
    pub look_at: NiLink<NiAVObject>,
}

impl Load for NiLookAtController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let look_at = stream.load()?;
        Ok(Self { base, look_at })
    }
}

impl Save for NiLookAtController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.look_at)?;
        Ok(())
    }
}

impl NiLookAtController {
    flag_props! {
        flip @ (mask = 0x0010) -> bool,
        axis @ (mask = 0x0060, pos = 5) -> Axis,
    }
}
