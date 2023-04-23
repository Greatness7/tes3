// external imports
use bytemuck::cast_slice;

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiFloatKey {
    #[default]
    LinKey(Vec<NiLinFloatKey>),
    BezKey(Vec<NiBezFloatKey>),
    TCBKey(Vec<NiTCBFloatKey>),
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiLinFloatKey {
    time: f32,
    value: f32,
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiBezFloatKey {
    time: f32,
    value: f32,
    in_tan: f32,
    out_an: f32,
}

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct NiTCBFloatKey {
    time: f32,
    value: f32,
    t: f32,
    c: f32,
    b: f32,
}

impl Load for NiFloatKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys: u32 = stream.load()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiFloatKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::BezKey => NiFloatKey::BezKey(stream.load_vec(num_keys)?),
            KeyType::TCBKey => NiFloatKey::TCBKey(stream.load_vec(num_keys)?),
            _ => Reader::error(format!("Invalid KeyType: {key_type:?}"))?,
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
        stream.save_as::<u32>(len)?;
        if !bytes.is_empty() {
            stream.save(&key_type)?;
            stream.save_bytes(bytes)?;
        }
        Ok(())
    }
}
