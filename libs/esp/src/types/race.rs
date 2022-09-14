// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Race {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: Option<String>,
    pub data: Option<RaceData>,
    pub spells: Option<Vec<String>>,
    pub description: Option<String>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct RaceData {
    pub skill_bonuses: SkillBonuses,
    pub strength: [i32; 2],
    pub intelligence: [i32; 2],
    pub willpower: [i32; 2],
    pub agility: [i32; 2],
    pub speed: [i32; 2],
    pub endurance: [i32; 2],
    pub personality: [i32; 2],
    pub luck: [i32; 2],
    pub height: [f32; 2],
    pub weight: [f32; 2],
    pub flags: u32,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct SkillBonuses {
    pub skill_0: SkillId,
    pub bonus_0: i32,
    pub skill_1: SkillId,
    pub bonus_1: i32,
    pub skill_2: SkillId,
    pub bonus_2: i32,
    pub skill_3: SkillId,
    pub bonus_3: i32,
    pub skill_4: SkillId,
    pub bonus_4: i32,
    pub skill_5: SkillId,
    pub bonus_5: i32,
    pub skill_6: SkillId,
    pub bonus_6: i32,
}

impl Load for Race {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"RADT" => {
                    stream.expect(140u32)?;
                    this.data = Some(stream.load()?);
                }
                b"NPCS" => {
                    this.spells.get_or_insert_with(default).push(stream.load()?);
                }
                b"DESC" => {
                    this.description = Some(stream.load()?);
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

impl Save for Race {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // RADT
        if let Some(value) = &self.data {
            stream.save(b"RADT")?;
            stream.save(&140u32)?;
            stream.save(value)?;
        }
        // NPCS
        for spell in self.spells.iter().flatten() {
            stream.save(b"NPCS")?;
            stream.save(spell)?;
        }
        // DESC
        if let Some(value) = &self.description {
            stream.save(b"DESC")?;
            stream.save(value)?;
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
