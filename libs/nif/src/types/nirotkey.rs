// external imports
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiRotKey {
    #[default]
    LinKey(Vec<NiLinRotKey>),
    BezKey(Vec<NiBezRotKey>),
    TCBKey(Vec<NiTCBRotKey>),
    EulerKey(EulerRotKey),
}

impl Load for NiRotKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiRotKey::LinKey(stream.load_seq(num_keys)?),
            KeyType::BezKey => NiRotKey::BezKey(stream.load_seq(num_keys)?),
            KeyType::TCBKey => NiRotKey::TCBKey(stream.load_seq(num_keys)?),
            KeyType::EulerKey => NiRotKey::EulerKey(stream.load()?),
            _ => Reader::error(format!("NiRotKey does not support {key_type:?}"))?,
        })
    }
}

impl Save for NiRotKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiRotKey::LinKey(keys) => {
                stream.save_as::<_, u32>(keys.len())?;
                if !keys.is_empty() {
                    stream.save(&KeyType::LinKey)?;
                    stream.save_seq(keys)?;
                }
            }
            NiRotKey::BezKey(keys) => {
                stream.save_as::<_, u32>(keys.len())?;
                if !keys.is_empty() {
                    stream.save(&KeyType::BezKey)?;
                    stream.save_seq(keys)?;
                }
            }
            NiRotKey::TCBKey(keys) => {
                stream.save_as::<_, u32>(keys.len())?;
                if !keys.is_empty() {
                    stream.save(&KeyType::TCBKey)?;
                    stream.save_seq(keys)?;
                }
            }
            NiRotKey::EulerKey(keys) => {
                let is_empty = keys.is_empty();
                stream.save_as::<_, u32>(!is_empty)?;
                if !is_empty {
                    stream.save(&KeyType::EulerKey)?;
                    stream.save(keys)?;
                }
            }
        };
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EulerRotKey {
    pub axis_order: AxisOrder,
    pub axes: [NiFloatData; 3],
}

impl EulerRotKey {
    pub fn is_empty(&self) -> bool {
        self.axes.iter().all(|axis| match &axis.keys {
            NiFloatKey::LinKey(keys) => keys.is_empty(),
            NiFloatKey::BezKey(keys) => keys.is_empty(),
            NiFloatKey::TCBKey(keys) => keys.is_empty(),
        })
    }
}

impl Load for EulerRotKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let axis_order = stream.load()?;
        let axes = [stream.load()?, stream.load()?, stream.load()?];
        Ok(Self { axis_order, axes })
    }
}

impl Save for EulerRotKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.axis_order)?;
        stream.save_seq(&self.axes)?;
        Ok(())
    }
}
