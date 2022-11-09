// external imports
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiFloatKey {
    #[default]
    LinKey(LinFloatKeys),
    BezKey(BezFloatKeys),
    TCBKey(TCBFloatKeys),
}

impl Load for NiFloatKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => LinFloatKeys::load(stream, num_keys)?.into(),
            KeyType::BezKey => BezFloatKeys::load(stream, num_keys)?.into(),
            KeyType::TCBKey => TCBFloatKeys::load(stream, num_keys)?.into(),
            _ => panic!("NiFloatKey does not support {key_type:?}"),
        })
    }
}

impl Save for NiFloatKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiFloatKey::LinKey(keys) => keys.save(stream),
            NiFloatKey::BezKey(keys) => keys.save(stream),
            NiFloatKey::TCBKey(keys) => keys.save(stream),
        }
    }
}
