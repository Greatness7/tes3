// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiTextKeyExtraData {
    pub base: NiExtraData,
    pub keys: Vec<NiTextKey>,
}

impl Load for NiTextKeyExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let keys = stream.load()?;
        Ok(Self { base, keys })
    }
}

impl Save for NiTextKeyExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.keys)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiTextKey {
    pub time: f32,
    pub value: String,
}

impl Load for NiTextKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let time = stream.load()?;
        let value = stream.load()?;
        Ok(Self { time, value })
    }
}

impl Save for NiTextKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.time)?;
        stream.save_string_without_null_terminator(&self.value)?;
        Ok(())
    }
}
