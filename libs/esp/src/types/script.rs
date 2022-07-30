// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Script {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub header: Option<ScriptHeader>,
    pub variables: Option<Vec<u8>>,
    pub bytecode: Option<Vec<u8>>,
    pub script_text: Option<String>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
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
                    this.header = Some(stream.load()?);
                }
                b"SCVR" => {
                    this.variables = Some(stream.load()?);
                }
                b"SCDT" => {
                    this.bytecode = Some(stream.load()?);
                }
                b"SCTX" => {
                    this.script_text = Some(stream.load()?);
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::Deleted);
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
        if let Some(value) = &self.header {
            stream.save(b"SCHD")?;
            stream.save(&52u32)?;
            stream.save::<FixedString<32>>(self.id.as_ref())?;
            stream.save(value)?;
        }
        // SCVR
        if let Some(value) = &self.variables {
            stream.save(b"SCVR")?;
            stream.save(value)?;
        }
        // SCDT
        if let Some(value) = &self.bytecode {
            stream.save(b"SCDT")?;
            stream.save(value)?;
        }
        // SCTX
        if let Some(value) = &self.script_text {
            stream.save(b"SCTX")?;
            stream.save(value)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::Deleted) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}
