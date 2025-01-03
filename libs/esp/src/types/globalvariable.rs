// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlobalVariable {
    pub flags: ObjectFlags,
    pub id: String,
    //pub global_type: GlobalType,
    pub value: GlobalValue,
}

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum GlobalValue {
    #[default]
    Float(f32),
    Short(i16),
    Long(i32),
}

impl Load for GlobalVariable {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        // this is guranteed to be loaded before FLTV according to Null
        let mut global_type = None;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    stream.expect(1u32)?;
                    global_type = Some(stream.load()?);
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    let mut val = stream.load::<f32>()?;
                    // Ignore NaNs, see "ratskilled" in "Morrowind.esm".
                    val = if val.is_nan() { 0.0 } else { val };
                    match global_type.expect("Incorrect FNAM order") {
                        GlobalType::Short => this.value = GlobalValue::Short(val as i16),
                        GlobalType::Long => this.value = GlobalValue::Long(val as i32),
                        GlobalType::Float => this.value = GlobalValue::Float(val),
                    }
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::DELETED);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for GlobalVariable {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        stream.save(b"FNAM")?;
        stream.save(&1u32)?;
        let global_type = match self.value {
            GlobalValue::Float(_) => GlobalType::Float,
            GlobalValue::Short(_) => GlobalType::Short,
            GlobalValue::Long(_) => GlobalType::Long,
        };
        stream.save(&global_type)?;
        // FLTV
        stream.save(b"FLTV")?;
        stream.save(&4u32)?;
        // save as f32
        let value = match self.value {
            GlobalValue::Float(value) => value,
            GlobalValue::Short(value) => value as f32,
            GlobalValue::Long(value) => value as f32,
        };
        stream.save(&value)?;
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
