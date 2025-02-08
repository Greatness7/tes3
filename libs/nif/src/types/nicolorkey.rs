// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, From, PartialEq, SmartDefault)]
pub enum NiColorKey {
    #[default]
    LinKey(Vec<NiLinColKey>),
}

#[derive(Meta, LoadSave, Clone, Copy, Debug, Default, PartialEq, Zeroable)]
pub struct NiLinColKey {
    pub time: f32,
    pub value: ColorA,
}

impl Load for NiColorKey {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let num_keys: u32 = stream.load()?;
        let key_type = if num_keys == 0 { KeyType::LinKey } else { stream.load()? };
        Ok(match key_type {
            KeyType::LinKey => NiColorKey::LinKey(stream.load_seq(num_keys)?),
            _ => Reader::error(format!("Invalid KeyType: {key_type:?}"))?,
        })
    }
}

impl Save for NiColorKey {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            NiColorKey::LinKey(keys) => {
                stream.save_as::<u32>(keys.len())?;
                if !keys.is_empty() {
                    stream.save(&KeyType::LinKey)?;
                    stream.save_seq(keys)?;
                }
            }
        }
        Ok(())
    }
}
