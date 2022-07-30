// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Faction {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub name: Option<String>,
    pub rank_names: Option<Vec<FixedString<32>>>,
    pub data: Option<FactionData>,
    pub reactions: Option<Vec<FactionReaction>>,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionData {
    pub favored_attributes: [AttributeId; 2],
    pub requirements: [FactionRequirement; 10],
    pub favored_skills: [SkillId; 7],
    pub flags: u32,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionRequirement {
    pub attributes: [i32; 2],
    pub primary_skill: i32,
    pub favored_skill: i32,
    pub reputation: i32,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct FactionReaction {
    pub faction: String,
    pub reaction: i32,
}

impl Load for Faction {
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
                b"RNAM" => {
                    stream.expect(32u32)?;
                    this.rank_names.get_or_insert_with(default).push(stream.load()?);
                }
                b"FADT" => {
                    stream.expect(240u32)?;
                    this.data = Some(stream.load()?);
                }
                b"ANAM" => {
                    this.reactions.get_or_insert_with(default).push(stream.load()?);
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

impl Save for Faction {
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
        // RNAM
        for value in self.rank_names.iter().flatten() {
            stream.save(b"RNAM")?;
            stream.save(&32u32)?;
            stream.save(value)?;
        }
        // FADT
        if let Some(value) = &self.data {
            stream.save(b"FADT")?;
            stream.save(&240u32)?;
            stream.save(value)?;
        }
        // ANAM / INTV
        for reaction in self.reactions.iter().flatten() {
            stream.save(reaction)?;
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

impl Load for FactionData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let favored_attributes = stream.load()?;
        let requirements = [
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
            stream.load()?,
        ];
        let favored_skills = stream.load()?;
        let flags = stream.load()?;
        Ok(Self {
            favored_attributes,
            requirements,
            favored_skills,
            flags,
        })
    }
}

impl Save for FactionData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.favored_attributes)?;
        for requirement in &self.requirements {
            stream.save(requirement)?;
        }
        stream.save(&self.favored_skills)?;
        stream.save(&self.flags)?;
        Ok(())
    }
}

impl Load for FactionReaction {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        // ANAM
        let faction = stream.load()?;
        // INTV
        stream.expect(*b"INTV")?;
        stream.expect(4u32)?;
        let reaction = stream.load()?;
        Ok(Self { faction, reaction })
    }
}

impl Save for FactionReaction {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // ANAM
        stream.save(b"ANAM")?;
        stream.save(&self.faction)?;
        // INTV
        stream.save(b"INTV")?;
        stream.save(&4u32)?;
        stream.save(&self.reaction)?;
        Ok(())
    }
}
