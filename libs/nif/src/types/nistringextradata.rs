// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiStringExtraData {
    pub base: NiExtraData,
    pub value: String,
}

impl Load for NiStringExtraData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let value = stream.load()?;
        Ok(Self { base, value })
    }
}

impl Save for NiStringExtraData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_string_without_null_terminator(&self.value)?;
        Ok(())
    }
}

impl NiStringExtraData {
    pub fn starts_with_ignore_ascii_case(&self, prefix: &str) -> bool {
        self.value
            .bytes()
            .zip(prefix.bytes())
            .all(|(a, b)| a.eq_ignore_ascii_case(&b))
    }
}
