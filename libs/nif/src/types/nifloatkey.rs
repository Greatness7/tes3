// external imports
use bytemuck::cast_slice;
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiFloatKey {
    #[default]
    LinKey(Vec<NiLinFloatKey>),
    BezKey(Vec<NiBezFloatKey>),
    TCBKey(Vec<NiTCBFloatKey>),
}

impl Load for NiFloatKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiFloatKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::BezKey => NiFloatKey::BezKey(stream.load_vec(num_keys)?),
            KeyType::TCBKey => NiFloatKey::TCBKey(stream.load_vec(num_keys)?),
            _ => Reader::error(format!("NiFloatKey does not support {key_type:?}"))?,
        })
    }
}

impl Save for NiFloatKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        let (len, key_type, bytes) = match self {
            NiFloatKey::LinKey(keys) => (keys.len(), KeyType::LinKey, cast_slice(keys)),
            NiFloatKey::BezKey(keys) => (keys.len(), KeyType::BezKey, cast_slice(keys)),
            NiFloatKey::TCBKey(keys) => (keys.len(), KeyType::TCBKey, cast_slice(keys)),
        };
        stream.save_as::<_, u32>(len)?;
        if !bytes.is_empty() {
            stream.save(&key_type)?;
            stream.save_bytes(bytes)?;
        }
        Ok(())
    }
}
