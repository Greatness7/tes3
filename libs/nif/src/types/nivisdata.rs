// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiVisData {
    pub base: NiObject,
    pub keys: Vec<NiVisKey>,
}

#[derive(Meta, LoadSave, Clone, Copy, Debug, Default, PartialEq, Zeroable)]
pub struct NiVisKey {
    pub time: f32,
    pub value: u8,
}

impl Load for NiVisData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_keys: u32 = stream.load()?;
        let keys = stream.load_seq(num_keys)?;
        Ok(Self { base, keys })
    }
}

impl Save for NiVisData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u32>(self.keys.len())?;
        stream.save_seq(&self.keys)?;
        Ok(())
    }
}
