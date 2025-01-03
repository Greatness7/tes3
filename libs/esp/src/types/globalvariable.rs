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

        //  MW is very lenient with subrecord orderm, so it might be a case where you load the value before loadign the type
        let mut temp_val = 0.0;
        let mut global_type = GlobalType::Short;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    stream.expect(1u32)?;
                    global_type = stream.load()?;
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    temp_val = stream.load::<f32>()?;
                    // Ignore NaNs, see "ratskilled" in "Morrowind.esm".
                    temp_val = if temp_val.is_nan() { 0.0 } else { temp_val };
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

        // now save the temp value
        match global_type {
            GlobalType::Short => this.value = GlobalValue::Short(temp_val as i16),
            GlobalType::Long => this.value = GlobalValue::Long(temp_val as i32),
            GlobalType::Float => this.value = GlobalValue::Float(temp_val),
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
