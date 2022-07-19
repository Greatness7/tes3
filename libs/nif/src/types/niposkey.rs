// external imports
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiPosKey {
    #[default]
    LinKey(LinPosKeys),
    BezKey(BezPosKeys),
    TCBKey(TCBPosKeys),
}

impl Load for NiPosKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => LinPosKeys::load(stream, num_keys)?.into(),
            KeyType::BezKey => BezPosKeys::load(stream, num_keys)?.into(),
            KeyType::TCBKey => TCBPosKeys::load(stream, num_keys)?.into(),
            _ => panic!("NiPosKey does not support {:?}", key_type),
        })
    }
}

impl Save for NiPosKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiPosKey::LinKey(keys) => keys.save(stream),
            NiPosKey::BezKey(keys) => keys.save(stream),
            NiPosKey::TCBKey(keys) => keys.save(stream),
        }
    }
}
