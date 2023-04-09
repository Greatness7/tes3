// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Script {
    pub flags: ObjectFlags,
    pub id: String,
    pub header: ScriptHeader,
    pub variables: Vec<u8>,
    pub bytecode: Vec<u8>,
    pub script_text: String,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct ScriptHeader {
    pub num_shorts: u32,
    pub num_longs: u32,
    pub num_floats: u32,
    pub bytecode_length: u32,
    pub variables_length: u32,
}

impl Load for Script {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"SCHD" => {
                    stream.expect(52u32)?;
                    this.id = stream.load::<FixedString<32>>()?.into();
                    this.header = stream.load()?;
                }
                b"SCVR" => {
                    this.variables = stream.load()?;
                }
                b"SCDT" => {
                    this.bytecode = stream.load()?;
                }
                b"SCTX" => {
                    this.script_text = stream.load()?;
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

impl Save for Script {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // SCHD
        stream.save(b"SCHD")?;
        stream.save(&52u32)?;
        stream.save::<FixedString<32>>(self.id.as_ref())?;
        stream.save(&self.header)?;
        // SCVR
        if !self.variables.is_empty() {
            stream.save(b"SCVR")?;
            stream.save(&self.variables)?;
        }
        // SCDT
        if !self.bytecode.is_empty() {
            stream.save(b"SCDT")?;
            stream.save(&self.bytecode)?;
        }
        // SCTX
        if !self.script_text.is_empty() {
            stream.save(b"SCTX")?;
            stream.save(&self.script_text)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
