// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiVisData {
    pub base: NiObject,
    pub keys: Vec<NiVisKey>,
}

impl Load for NiVisData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let keys = stream.load()?;
        Ok(Self { base, keys })
    }
}

impl Save for NiVisData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.keys)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiVisKey {
    pub time: f32,
    pub value: u8,
}

impl Load for NiVisKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let time = stream.load()?;
        let value = stream.load()?;
        Ok(Self { time, value })
    }
}

impl Save for NiVisKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.time)?;
        stream.save(&self.value)?;
        Ok(())
    }
}
