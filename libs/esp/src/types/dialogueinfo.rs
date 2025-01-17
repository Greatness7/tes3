// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DialogueInfo {
    pub flags: ObjectFlags,
    pub id: String,
    pub prev_id: String,
    pub next_id: String,
    pub data: DialogueData,
    pub speaker_id: String,
    pub speaker_race: String,
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
pub struct DialogueData {
    pub dialogue_type: DialogueType,
    pub disposition: i32,
    pub speaker_rank: i8,
    pub speaker_sex: Sex,
    pub player_rank: i8,
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filter {
    pub index: u8,
    pub filter_type: FilterType,
    pub function: FilterFunction,
    pub comparison: FilterComparison,
    pub id: String,
    pub value: FilterValue,
}

#[esp_meta]
#[derive(Clone, Copy, Debug, PartialEq, SmartDefault)]
pub enum FilterValue {
    #[default]
    Float(f32),
    Integer(i32),
}

#[esp_meta]
#[derive(Clone, Copy, Debug, Eq, PartialEq, SmartDefault)]
pub enum QuestState {
    #[default]
    Name = 0,
    Finished = 1,
    Restart = 2,
}

impl Load for DialogueInfo {
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
                    this.speaker_race = stream.load()?;
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

impl Save for DialogueInfo {
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
        if !self.speaker_race.is_empty() {
            stream.save(b"RNAM")?;
            stream.save(&self.speaker_race)?;
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
            // There's an engine limit of 512 characters for this field.
            // Don't include null terminators as they might push us over the limit.
            stream.save_string_without_null_terminator(&self.text)?;
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

impl Load for DialogueData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let dialogue_type = stream.load()?;
        let disposition = stream.load()?;
        let speaker_rank = stream.load()?;
        let speaker_sex = stream.load()?;
        let player_rank = stream.load()?;
        stream.skip(1)?; // padding
        Ok(Self {
            dialogue_type,
            disposition,
            speaker_rank,
            speaker_sex,
            player_rank,
        })
    }
}

impl Save for DialogueData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.dialogue_type)?;
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
        let len = stream.load_as::<u32, usize>()?;
        let index = stream.load::<u8>()?;
        let filter_type = stream.load()?;
        let function = stream.load()?;
        let comparison = stream.load()?;
        let id = stream.load_string(len - 5)?;
        let value = default();
        // Convert the index from a char to a number for convenience.
        let Some(index) = index.checked_sub(b'0') else {
            return Reader::error("DialogueInfo: Invalid filter index");
        };
        Ok(Self {
            index,
            filter_type,
            function,
            comparison,
            id,
            value,
        })
    }
}

impl Save for Filter {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // Convert the index back to a char as required by the format.
        let Some(index) = self.index.checked_add(b'0') else {
            return Writer::error("DialogueInfo: Invalid filter index");
        };
        let id = stream.encode(&self.id)?;
        stream.save_as::<u32>(id.len() + 5)?;
        stream.save(&index)?;
        stream.save(&self.filter_type)?;
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
