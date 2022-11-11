// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Info {
    pub flags: ObjectFlags,
    pub id: String,
    pub prev_id: String,
    pub next_id: String,
    pub data: InfoData,
    pub speaker_id: String,
    pub speaker_rank: String,
    pub speaker_class: String,
    pub speaker_faction: String,
    pub speaker_cell: String,
    pub player_faction: String,
    pub sound_path: String,
    pub text: String,
    pub quest_state: Option<QuestState>,
    pub filters: Vec<Filter>,
    pub script_text: String,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InfoData {
    pub kind: DialogueType,
    pub disposition: i32,
    pub speaker_rank: i8,
    pub speaker_sex: Sex,
    pub player_rank: i8,
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filter {
    pub slot: FilterSlot,
    pub kind: FilterType,
    pub function: FilterFunction,
    pub comparison: FilterComparison,
    pub id: String,
    pub value: FilterValue,
}

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum FilterValue {
    #[default]
    Float(f32),
    Integer(i32),
}

#[esp_meta]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestState {
    Name,
    Finished,
    Restart,
}

impl Load for Info {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INAM" => {
                    this.id = stream.load()?;
                }
                b"PNAM" => {
                    this.prev_id = stream.load()?;
                }
                b"NNAM" => {
                    this.next_id = stream.load()?;
                }
                b"DATA" => {
                    stream.expect(12u32)?;
                    this.data = stream.load()?;
                }
                b"ONAM" => {
                    this.speaker_id = stream.load()?;
                }
                b"RNAM" => {
                    this.speaker_rank = stream.load()?;
                }
                b"CNAM" => {
                    this.speaker_class = stream.load()?;
                }
                b"FNAM" => {
                    this.speaker_faction = stream.load()?;
                }
                b"ANAM" => {
                    this.speaker_cell = stream.load()?;
                }
                b"DNAM" => {
                    this.player_faction = stream.load()?;
                }
                b"SNAM" => {
                    this.sound_path = stream.load()?;
                }
                b"NAME" => {
                    this.text = stream.load()?;
                }
                b"QSTN" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.quest_state = Some(QuestState::Name);
                }
                b"QSTF" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.quest_state = Some(QuestState::Finished);
                }
                b"QSTR" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.quest_state = Some(QuestState::Restart);
                }
                b"SCVR" => {
                    this.filters.push(stream.load()?);
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    let filter = this.filters.last_mut().ok_or_else(err)?;
                    filter.value = FilterValue::Float(stream.load()?);
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    let filter = this.filters.last_mut().ok_or_else(err)?;
                    filter.value = FilterValue::Integer(stream.load()?);
                }
                b"BNAM" => {
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

impl Save for Info {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // INAM
        stream.save(b"INAM")?;
        stream.save(&self.id)?;
        // PNAM
        stream.save(b"PNAM")?;
        stream.save(&self.prev_id)?;
        // NNAM
        stream.save(b"NNAM")?;
        stream.save(&self.next_id)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&12u32)?;
        stream.save(&self.data)?;
        // ONAM
        if !self.speaker_id.is_empty() {
            stream.save(b"ONAM")?;
            stream.save(&self.speaker_id)?;
        }
        // RNAM
        if !self.speaker_rank.is_empty() {
            stream.save(b"RNAM")?;
            stream.save(&self.speaker_rank)?;
        }
        // CNAM
        if !self.speaker_class.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.speaker_class)?;
        }
        // FNAM
        if !self.speaker_faction.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.speaker_faction)?;
        }
        // ANAM
        if !self.speaker_cell.is_empty() {
            stream.save(b"ANAM")?;
            stream.save(&self.speaker_cell)?;
        }
        // DNAM
        if !self.player_faction.is_empty() {
            stream.save(b"DNAM")?;
            stream.save(&self.player_faction)?;
        }
        // SNAM
        if !self.sound_path.is_empty() {
            stream.save(b"SNAM")?;
            stream.save(&self.sound_path)?;
        }
        // NAME
        if !self.text.is_empty() {
            stream.save(b"NAME")?;
            stream.save(&self.text)?;
        }
        //
        match self.quest_state {
            // QSTN
            Some(QuestState::Name) => {
                stream.save(b"QSTN")?;
                stream.save(&1u32)?;
                stream.save(&1u8)?;
            }
            // QSTF
            Some(QuestState::Finished) => {
                stream.save(b"QSTF")?;
                stream.save(&1u32)?;
                stream.save(&1u8)?;
            }
            // QSTR
            Some(QuestState::Restart) => {
                stream.save(b"QSTR")?;
                stream.save(&1u32)?;
                stream.save(&1u8)?;
            }
            _ => {}
        }
        //
        for filter in &self.filters {
            // SCVR
            stream.save(b"SCVR")?;
            stream.save(filter)?;
            match &filter.value {
                FilterValue::Float(value) => {
                    // FLTV
                    stream.save(b"FLTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
                FilterValue::Integer(value) => {
                    // INTV
                    stream.save(b"INTV")?;
                    stream.save(&4u32)?;
                    stream.save(value)?;
                }
            }
        }
        // BNAM
        if !self.script_text.is_empty() {
            stream.save(b"BNAM")?;
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
        let value = default();
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
