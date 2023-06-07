// external imports
use bytemuck::cast_slice;

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiPosKey {
    #[default]
    LinKey(Vec<NiLinPosKey>),
    BezKey(Vec<NiBezPosKey>),
    TCBKey(Vec<NiTCBPosKey>),
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiLinPosKey {
    pub time: f32,
    pub value: Vec3,
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiBezPosKey {
    pub time: f32,
    pub value: Vec3,
    pub in_tan: Vec3,
    pub out_tan: Vec3,
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiTCBPosKey {
    pub time: f32,
    pub value: Vec3,
    pub tension: f32,
    pub continuity: f32,
    pub bias: f32,
}

impl Load for NiPosKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys: u32 = stream.load()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiPosKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::BezKey => NiPosKey::BezKey(stream.load_vec(num_keys)?),
            KeyType::TCBKey => NiPosKey::TCBKey(stream.load_vec(num_keys)?),
            _ => Reader::error(format!("Invalid KeyType: {key_type:?}"))?,
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
        stream.save_as::<u32>(len)?;
        if !bytes.is_empty() {
            stream.save(&key_type)?;
            stream.save_bytes(bytes)?;
        }
        Ok(())
    }
}
