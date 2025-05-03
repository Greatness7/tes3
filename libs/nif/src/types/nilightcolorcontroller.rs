// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiLightColorController {
    pub base: NiTimeController,
    pub data: NiLink<NiPosData>,
}

impl Load for NiLightColorController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let data = stream.load()?;
        Ok(Self { base, data })
    }
}

impl Save for NiLightColorController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.data)?;
        Ok(())
    }
}

impl NiLightColorController {
    flag_props! {
        ambient @ (mask = 0x0010) -> bool,
    }
}
