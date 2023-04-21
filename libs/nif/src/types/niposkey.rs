// external imports
use bytemuck::cast_slice;
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiPosKey {
    #[default]
    LinKey(Vec<NiLinPosKey>),
    BezKey(Vec<NiBezPosKey>),
    TCBKey(Vec<NiTCBPosKey>),
}

impl Load for NiPosKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiPosKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::BezKey => NiPosKey::BezKey(stream.load_vec(num_keys)?),
            KeyType::TCBKey => NiPosKey::TCBKey(stream.load_vec(num_keys)?),
            _ => Reader::error(format!("NiPosKey does not support {key_type:?}"))?,
        })
    }
}

impl Save for NiPosKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        let (len, key_type, bytes) = match self {
            NiPosKey::LinKey(keys) => (keys.len(), KeyType::LinKey, cast_slice(keys)),
            NiPosKey::BezKey(keys) => (keys.len(), KeyType::BezKey, cast_slice(keys)),
            NiPosKey::TCBKey(keys) => (keys.len(), KeyType::TCBKey, cast_slice(keys)),
        };
        stream.save_as::<_, u32>(len)?;
        if !bytes.is_empty() {
            stream.save(&key_type)?;
            stream.save_bytes(bytes)?;
        }
        Ok(())
    }
}
