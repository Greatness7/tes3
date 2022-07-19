// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiKeyframeManager {
    pub base: NiTimeController,
    pub sequences: Vec<NiSequence>,
}

impl Load for NiKeyframeManager {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let sequences = stream.load()?;
        Ok(Self { base, sequences })
    }
}

impl Save for NiKeyframeManager {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.sequences)?;
        Ok(())
    }
}
