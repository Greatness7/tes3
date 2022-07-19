// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiObjectNET {
    pub base: NiObject,
    pub name: String,
    pub extra_data: NiLink<NiExtraData>,
    pub controller: NiLink<NiTimeController>,
}

impl Load for NiObjectNET {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let name = stream.load()?;
        let extra_data = stream.load()?;
        let controller = stream.load()?;
        Ok(Self {
            base,
            name,
            extra_data,
            controller,
        })
    }
}

impl Save for NiObjectNET {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.name)?;
        stream.save(&self.extra_data)?;
        stream.save(&self.controller)?;
        Ok(())
    }
}
