// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct GameSetting {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub value: Option<GameSettingValue>,
}

#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum GameSettingValue {
    #[default]
    String(String),
    Float(f32),
    Integer(i32),
}

impl Load for GameSetting {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"STRV" => {
                    this.value = Some(GameSettingValue::String(stream.load()?));
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    this.value = Some(GameSettingValue::Float(stream.load()?));
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    this.value = Some(GameSettingValue::Integer(stream.load()?));
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for GameSetting {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        //
        if let Some(value) = &self.value {
            match value {
                GameSettingValue::String(value) => {
                    // STRV
                    stream.save(b"STRV")?;
                    stream.save(value)?;
                }
                GameSettingValue::Float(value) => {
                    // FLTV
                    stream.save(b"FLTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
                GameSettingValue::Integer(value) => {
                    // INTV
                    stream.save(b"INTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
            }
        }
        Ok(())
    }
}
