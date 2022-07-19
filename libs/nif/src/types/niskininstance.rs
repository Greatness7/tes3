// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSkinInstance {
    pub base: NiObject,
    pub data: NiLink<NiSkinData>,
    pub root: NiLink<NiAVObject>,
    pub bones: Vec<NiLink<NiAVObject>>,
}

impl Load for NiSkinInstance {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let data = stream.load()?;
        let root = stream.load()?;
        let bones = stream.load()?;
        Ok(Self { base, data, root, bones })
    }
}

impl Save for NiSkinInstance {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.data)?;
        stream.save(&self.root)?;
        stream.save(&self.bones)?;
        Ok(())
    }
}
