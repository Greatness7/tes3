// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub prev_id: Option<String>,
    pub next_id: Option<String>,
    pub data: Option<InfoData>,
    pub speaker_id: Option<String>,
    pub speaker_rank: Option<String>,
    pub speaker_class: Option<String>,
    pub speaker_faction: Option<String>,
    pub speaker_cell: Option<String>,
    pub player_faction: Option<String>,
    pub text: Option<String>,
    pub sound_path: Option<String>,
    pub quest_name: Option<u8>,
    pub quest_finish: Option<u8>,
    pub quest_restart: Option<u8>,
    pub filters: Option<Vec<Filter>>,
    pub script_text: Option<String>,
    pub deleted: Option<u32>,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct InfoData {
    pub kind: DialogueType,
    pub disposition: i32,
    pub speaker_rank: i8,
    pub speaker_sex: Sex,
    pub player_rank: i8,
}

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Filter {
    pub slot: FilterSlot,
    pub kind: FilterType,
    pub function: FilterFunction,
    pub comparison: FilterComparison,
    pub id: String,
    pub value: Option<FilterValue>,
}

#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum FilterValue {
    #[default]
    Float(f32),
    Integer(i32),
}

impl Load for Info {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INAM" => {
                    this.id = stream.load()?;
                }
                b"PNAM" => {
                    this.prev_id = Some(stream.load()?);
                }
                b"NNAM" => {
                    this.next_id = Some(stream.load()?);
                }
                b"DATA" => {
                    stream.expect(12u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ONAM" => {
                    this.speaker_id = Some(stream.load()?);
                }
                b"RNAM" => {
                    this.speaker_rank = Some(stream.load()?);
                }
                b"CNAM" => {
                    this.speaker_class = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.speaker_faction = Some(stream.load()?);
                }
                b"ANAM" => {
                    this.speaker_cell = Some(stream.load()?);
                }
                b"DNAM" => {
                    this.player_faction = Some(stream.load()?);
                }
                b"SNAM" => {
                    this.sound_path = Some(stream.load()?);
                }
                b"NAME" => {
                    this.text = Some(stream.load()?);
                }
                b"QSTN" => {
                    stream.expect(1u32)?;
                    this.quest_name = Some(stream.load()?);
                }
                b"QSTF" => {
                    stream.expect(1u32)?;
                    this.quest_finish = Some(stream.load()?);
                }
                b"QSTR" => {
                    stream.expect(1u32)?;
                    this.quest_restart = Some(stream.load()?);
                }
                b"SCVR" => {
                    this.filters.get_or_insert_default().push(stream.load()?);
                }
                b"FLTV" => {
                    // TODO these most likely follow immmediately after
                    stream.expect(4u32)?;
                    let filter = this.filters.get_or_insert_default().last_mut().ok_or_else(err)?;
                    filter.value = Some(FilterValue::Float(stream.load()?));
                }
                b"INTV" => {
                    // TODO these most likely follow immmediately after
                    stream.expect(4u32)?;
                    let filter = this.filters.get_or_insert_default().last_mut().ok_or_else(err)?;
                    filter.value = Some(FilterValue::Integer(stream.load()?));
                }
                b"BNAM" => {
                    this.script_text = Some(stream.load()?);
                }
                b"DELE" => {
                    stream.expect(4u32)?;
                    this.deleted = Some(stream.load()?);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Info {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // INAM
        stream.save(b"INAM")?;
        stream.save(&self.id)?;
        // PNAM
        if let Some(value) = &self.prev_id {
            stream.save(b"PNAM")?;
            stream.save(value)?;
        }
        // NNAM
        if let Some(value) = &self.next_id {
            stream.save(b"NNAM")?;
            stream.save(value)?;
        }
        // DATA
        if let Some(value) = &self.data {
            stream.save(b"DATA")?;
            stream.save(&12u32)?;
            stream.save(value)?;
        }
        // ONAM
        if let Some(value) = &self.speaker_id {
            stream.save(b"ONAM")?;
            stream.save(value)?;
        }
        // RNAM
        if let Some(value) = &self.speaker_rank {
            stream.save(b"RNAM")?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.speaker_class {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.speaker_faction {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // ANAM
        if let Some(value) = &self.speaker_cell {
            stream.save(b"ANAM")?;
            stream.save(value)?;
        }
        // DNAM
        if let Some(value) = &self.player_faction {
            stream.save(b"DNAM")?;
            stream.save(value)?;
        }
        // SNAM
        if let Some(value) = &self.sound_path {
            stream.save(b"SNAM")?;
            stream.save(value)?;
        }
        // NAME
        if let Some(value) = &self.text {
            stream.save(b"NAME")?;
            stream.save(value)?;
        }
        // QSTN
        if let Some(value) = &self.quest_name {
            stream.save(b"QSTN")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        // QSTF
        if let Some(value) = &self.quest_finish {
            stream.save(b"QSTF")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        // QSTR
        if let Some(value) = &self.quest_restart {
            stream.save(b"QSTR")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        //
        for filter in self.filters.iter().flatten() {
            // SCVR
            stream.save(b"SCVR")?;
            stream.save(filter)?;
            match &filter.value {
                Some(FilterValue::Float(value)) => {
                    // FLTV
                    stream.save(b"FLTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
                Some(FilterValue::Integer(value)) => {
                    // INTV
                    stream.save(b"INTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
                _ => {}
            }
        }
        // BNAM
        if let Some(value) = &self.script_text {
            stream.save(b"BNAM")?;
            stream.save(value)?;
        }
        // DELE
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}

impl Load for InfoData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let kind = stream.load()?;
        let disposition = stream.load()?;
        let speaker_rank = stream.load()?;
        let speaker_sex = stream.load()?;
        let player_rank = stream.load()?;
        stream.skip(1)?; // padding
        Ok(Self {
            kind,
            disposition,
            speaker_rank,
            speaker_sex,
            player_rank,
        })
    }
}

impl Save for InfoData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.kind)?;
        stream.save(&self.disposition)?;
        stream.save(&self.speaker_rank)?;
        stream.save(&self.speaker_sex)?;
        stream.save(&self.player_rank)?;
        stream.save(&[0u8; 1])?; // padding
        Ok(())
    }
}

impl Load for Filter {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len: u32 = stream.load()?;
        let slot = stream.load()?;
        let kind = stream.load()?;
        let function = stream.load()?;
        let comparison = stream.load()?;
        let id = stream.load_string(len as usize - 5)?;
        let value = None;
        Ok(Self {
            slot,
            kind,
            function,
            comparison,
            id,
            value,
        })
    }
}

impl Save for Filter {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        let id = stream.encode(&self.id)?;
        stream.save_as::<_, u32>(id.len() + 5)?;
        stream.save(&self.slot)?;
        stream.save(&self.kind)?;
        stream.save(&self.function)?;
        stream.save(&self.comparison)?;
        stream.save_bytes(&id)?;
        Ok(())
    }
}

fn err() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "Info filter value was provided without a corresponding filter definition.",
    )
}
