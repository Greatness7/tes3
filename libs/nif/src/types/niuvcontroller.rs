// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiUVController {
    pub base: NiTimeController,
    pub texture_set: u16,
    pub data: NiLink<NiUVData>,
}

impl Load for NiUVController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let texture_set = stream.load()?;
        let data = stream.load()?;
        Ok(Self { base, texture_set, data })
    }
}

impl Save for NiUVController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.texture_set)?;
        stream.save(&self.data)?;
        Ok(())
    }
}
