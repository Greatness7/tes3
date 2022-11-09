// external imports
use derive_more::From;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiRotKey {
    #[default]
    LinKey(LinRotKeys),
    BezKey(BezRotKeys),
    TCBKey(TCBRotKeys),
    EulerKey(EulerRotKeys),
}

impl Load for NiRotKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => LinRotKeys::load_q(stream, num_keys)?.into(),
            KeyType::BezKey => BezRotKeys::load_q(stream, num_keys)?.into(),
            KeyType::TCBKey => TCBRotKeys::load_q(stream, num_keys)?.into(),
            KeyType::EulerKey => EulerRotKeys::load(stream)?.into(),
            KeyType::NoInterp => panic!("NiRotKey does not support {key_type:?}"),
        })
    }
}

impl Save for NiRotKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiRotKey::LinKey(keys) => keys.save_q(stream)?,
            NiRotKey::BezKey(keys) => keys.save_q(stream)?,
            NiRotKey::TCBKey(keys) => keys.save_q(stream)?,
            NiRotKey::EulerKey(keys) => keys.save(stream)?,
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EulerRotKeys {
    pub euler_axis_order: AxisOrder,
    pub euler_data: [NiFloatData; 3],
}

impl EulerRotKeys {
    fn has_keys(&self) -> bool {
        for data in &self.euler_data {
            if !match &data.keys {
                NiFloatKey::LinKey(keys) => keys.is_empty(),
                NiFloatKey::BezKey(keys) => keys.is_empty(),
                NiFloatKey::TCBKey(keys) => keys.is_empty(),
            } {
                return true;
            }
        }
        false
    }
}

impl Load for EulerRotKeys {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let euler_axis_order = stream.load()?;
        let euler_data = [stream.load()?, stream.load()?, stream.load()?];
        Ok(Self {
            euler_axis_order,
            euler_data,
        })
    }
}

impl Save for EulerRotKeys {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        if self.has_keys() {
            stream.save(&1u32)?;
            stream.save(&KeyType::EulerKey)?;
            stream.save(&self.euler_axis_order)?;
            stream.save(&self.euler_data[0])?;
            stream.save(&self.euler_data[1])?;
            stream.save(&self.euler_data[2])?;
        } else {
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
