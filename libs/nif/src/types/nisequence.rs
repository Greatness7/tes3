// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiSequence {
    pub base: NiObject,
    pub sequence_name: String,
    #[default(SequenceTarget::Internal(0, 0))]
    pub sequence_target: SequenceTarget,
    pub name_controller_pairs: Vec<(String, i32)>,
}

impl Load for NiSequence {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let sequence_name = stream.load()?;
        let sequence_target = stream.load()?;
        let num_name_controller_pairs: u32 = stream.load()?;
        let name_controller_pairs = stream.load_seq(num_name_controller_pairs)?;
        Ok(Self {
            base,
            sequence_name,
            sequence_target,
            name_controller_pairs,
        })
    }
}

impl Save for NiSequence {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_string_without_null_terminator(&self.sequence_name)?;
        stream.save(&self.sequence_target)?;
        stream.save_as::<u32>(self.name_controller_pairs.len())?;
        for (name, controller) in &self.name_controller_pairs {
            stream.save_string_without_null_terminator(name)?;
            stream.save(controller)?;
        }
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub enum SequenceTarget {
    #[default]
    External(String),
    Internal(i32, i32),
}

impl Load for SequenceTarget {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let has_external = stream.load::<u8>()? != 0;
        if has_external {
            Ok(SequenceTarget::External(stream.load()?))
        } else {
            Ok(SequenceTarget::Internal(stream.load()?, stream.load()?))
        }
    }
}

impl Save for SequenceTarget {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        match self {
            SequenceTarget::External(file_name) => {
                stream.save(&1u8)?; // has_external
                stream.save_string_without_null_terminator(file_name)?;
            }
            SequenceTarget::Internal(unknown1, unknown2) => {
                stream.save(&0u8)?; // has_external
                stream.save(unknown1)?;
                stream.save(unknown2)?;
            }
        }
        Ok(())
    }
}
