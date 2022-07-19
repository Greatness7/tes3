// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiKeyframeData {
    pub base: NiObject,
    pub rotations: NiRotData,
    pub translations: NiPosData,
    pub scales: NiFloatData,
}

impl Load for NiKeyframeData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let rotations = stream.load()?;
        let translations = stream.load()?;
        let scales = stream.load()?;
        Ok(Self {
            base,
            rotations,
            translations,
            scales,
        })
    }
}

impl Save for NiKeyframeData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.rotations)?;
        stream.save(&self.translations)?;
        stream.save(&self.scales)?;
        Ok(())
    }
}
