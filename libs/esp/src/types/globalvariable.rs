// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlobalVariable {
    pub flags: ObjectFlags,
    pub id: String,
    pub value: GlobalValue,
}

#[esp_meta]
#[derive(Clone, Copy, Debug, PartialEq, SmartDefault)]
pub enum GlobalValue {
    #[default]
    Float(f32),
    Long(i32),
    Short(i16),
}

impl Load for GlobalVariable {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    stream.expect(1u32)?;
                    let global_type = stream.load()?;
                    stream.expect(*b"FLTV")?;
                    stream.expect(4u32)?;
                    let global_value = stream.load()?;
                    this.value = GlobalValue::from_f32(global_type, global_value);
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
        stream.save(&self.value.global_type())?;
        // FLTV
        stream.save(b"FLTV")?;
        stream.save(&4u32)?;
        stream.save(&self.value.to_f32())?;
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        // Disallow values with known precision errors.
        if self.value.has_precision_error() {
            Reader::error(format!("GlobalVariable precision error: {}", self.id))?;
        }
        Ok(())
    }
}

impl GlobalValue {
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_f32(t: GlobalType, v: f32) -> Self {
        // NaNs are converted to zero by the engine.
        // Example: "ratskilled" in "Morrowind.esm".
        let v = if v.is_nan() { 0.0 } else { v };
        match t {
            GlobalType::Float => GlobalValue::Float(v),
            GlobalType::Long => GlobalValue::Long(v as i32),
            GlobalType::Short => GlobalValue::Short(v as i16),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub const fn to_f32(self) -> f32 {
        match self {
            GlobalValue::Float(v) => v,
            GlobalValue::Long(v) => v as f32,
            GlobalValue::Short(v) => v as f32,
        }
    }

    pub const fn global_type(self) -> GlobalType {
        match self {
            GlobalValue::Float(_) => GlobalType::Float,
            GlobalValue::Long(_) => GlobalType::Long,
            GlobalValue::Short(_) => GlobalType::Short,
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub const fn has_precision_error(self) -> bool {
        match self {
            GlobalValue::Float(_) => false,
            GlobalValue::Long(v) => (v as f32 as i32) != v,
            GlobalValue::Short(v) => (v as f32 as i16) != v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_error() {
        let value = GlobalValue::Long(16777217);
        assert!(value.has_precision_error());
    }
}
