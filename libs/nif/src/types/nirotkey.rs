// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiRotKey {
    #[default]
    LinKey(Vec<NiLinRotKey>),
    BezKey(Vec<NiBezRotKey>),
    TCBKey(Vec<NiTCBRotKey>),
    EulerKey(NiEulerRotKeys),
}

#[derive(Meta, LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiLinRotKey {
    pub time: f32,
    #[default(Quat::IDENTITY)]
    pub value: Quat,
}

#[derive(Meta, LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiBezRotKey {
    pub time: f32,
    #[default(Quat::IDENTITY)]
    pub value: Quat,
}

#[derive(Meta, LoadSave, Clone, Copy, Debug, PartialEq, SmartDefault, Zeroable)]
pub struct NiTCBRotKey {
    pub time: f32,
    #[default(Quat::IDENTITY)]
    pub value: Quat,
    pub tension: f32,
    pub continuity: f32,
    pub bias: f32,
}

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiEulerRotKeys {
    pub axis_order: AxisOrder,
    pub axes: [NiFloatData; 3],
}

impl Load for NiRotKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys: u32 = stream.load()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiRotKey::LinKey(stream.load_seq(num_keys)?),
            KeyType::BezKey => NiRotKey::BezKey(stream.load_seq(num_keys)?),
            KeyType::TCBKey => NiRotKey::TCBKey(stream.load_seq(num_keys)?),
            KeyType::EulerKey => NiRotKey::EulerKey(stream.load()?),
            _ => Reader::error(format!("Invalid KeyType: {key_type:?}"))?,
        })
    }
}

impl Save for NiRotKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        /// Helper function to reduce code duplication in the match statement below.
        #[inline]
        fn save<'a, I, S>(stream: &mut Writer, len: usize, key_type: KeyType, keys: I) -> io::Result<()>
        where
            I: IntoIterator<Item = &'a S>,
            S: Save + 'a,
        {
            stream.save_as::<u32>(len)?;
            if len != 0 {
                stream.save(&key_type)?;
                stream.save_seq(keys)?;
            }
            Ok(())
        }
        match self {
            NiRotKey::LinKey(keys) => save(stream, keys.len(), KeyType::LinKey, keys),
            NiRotKey::BezKey(keys) => save(stream, keys.len(), KeyType::BezKey, keys),
            NiRotKey::TCBKey(keys) => save(stream, keys.len(), KeyType::TCBKey, keys),
            NiRotKey::EulerKey(keys) => save(stream, keys.len().min(1), KeyType::EulerKey, [keys]),
        }
    }
}

impl NiEulerRotKeys {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.axes
            .iter()
            .map(|axis| match &axis.keys {
                NiFloatKey::LinKey(keys) => keys.len(),
                NiFloatKey::BezKey(keys) => keys.len(),
                NiFloatKey::TCBKey(keys) => keys.len(),
            })
            .max()
            .unwrap_or(0)
    }
}

impl Load for NiEulerRotKeys {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let axis_order = stream.load()?;
        let axes = [stream.load()?, stream.load()?, stream.load()?];
        Ok(Self { axis_order, axes })
    }
}

impl Save for NiEulerRotKeys {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.axis_order)?;
        stream.save_seq(&self.axes)?;
        Ok(())
    }
}
