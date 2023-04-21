// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiBltSource {
    pub base: NiObject,
    #[default(TextureSource::Internal(NiLink::null()))]
    pub source: TextureSource,
}

impl Load for NiBltSource {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let source = stream.load()?;
        Ok(Self { base, source })
    }
}

impl Save for NiBltSource {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.source)?;
        Ok(())
    }
}
