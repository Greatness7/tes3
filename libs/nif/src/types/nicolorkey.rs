// external imports
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiColorKey {
    #[default]
    LinKey(LinColKeys),
}

impl Load for NiColorKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => LinColKeys::load(stream, num_keys)?.into(),
            _ => Reader::error(format!("NiColorKey does not support {key_type:?}"))?,
        })
    }
}

impl Save for NiColorKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiColorKey::LinKey(keys) => keys.save(stream),
        }
    }
}
