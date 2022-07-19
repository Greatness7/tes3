// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiFloatController {
    pub base: NiTimeController,
    pub data: NiLink<NiFloatData>,
}

impl Load for NiFloatController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let data = stream.load()?;
        Ok(Self { base, data })
    }
}

impl Save for NiFloatController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.data)?;
        Ok(())
    }
}
