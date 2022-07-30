// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Npc {
    pub flags: BitFlags<ObjectFlags>,
    pub id: String,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub script: Option<String>,
    pub race: Option<String>,
    pub class: Option<String>,
    pub faction: Option<String>,
    pub head: Option<String>,
    pub hair: Option<String>,
    pub npc_flags: Option<u32>,
    pub data: Option<NpcData>,
    pub inventory: Option<Vec<(i32, FixedString<32>)>>,
    pub spells: Option<Vec<String>>,
    pub ai_data: Option<AiData>,
    pub ai_packages: Option<Vec<AiPackage>>,
    pub travel_destinations: Option<Vec<TravelDestination>>,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct NpcData {
    pub level: i16,
    pub stats: Option<NpcStats>,
    pub disposition: i8,
    pub reputation: i8,
    pub rank: i8,
    pub gold: u32,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct NpcStats {
    pub attributes: [u8; 8],
    pub skills: [u8; 27],
    pub health: u16,
    pub magicka: u16,
    pub fatigue: u16,
}

impl Load for Npc {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"RNAM" => {
                    this.race = Some(stream.load()?);
                }
                b"CNAM" => {
                    this.class = Some(stream.load()?);
                }
                b"ANAM" => {
                    this.faction = Some(stream.load()?);
                }
                b"BNAM" => {
                    this.head = Some(stream.load()?);
                }
                b"KNAM" => {
                    this.hair = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"NPDT" => {
                    this.data = Some(stream.load()?);
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.npc_flags = Some(stream.load()?);
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.get_or_insert_with(default).push(stream.load()?);
                }
                b"NPCS" => {
                    this.spells.get_or_insert_with(default).push(stream.load()?);
                }
                b"AIDT" => {
                    stream.expect(12u32)?;
                    this.ai_data = Some(stream.load()?);
                }
                b"DODT" => {
                    stream.expect(24u32)?;
                    this.travel_destinations.get_or_insert_with(default).push(stream.load()?);
                }
                b"AI_T" => {
                    stream.expect(16u32)?;
                    this.ai_packages
                        .get_or_insert_with(default)
                        .push(AiPackage::Travel(stream.load()?));
                }
                b"AI_W" => {
                    stream.expect(14u32)?;
                    this.ai_packages
                        .get_or_insert_with(default)
                        .push(AiPackage::Wander(stream.load()?));
                }
                b"AI_E" => {
                    stream.expect(48u32)?;
                    this.ai_packages
                        .get_or_insert_with(default)
                        .push(AiPackage::Escort(stream.load()?));
                }
                b"AI_F" => {
                    stream.expect(48u32)?;
                    this.ai_packages
                        .get_or_insert_with(default)
                        .push(AiPackage::Follow(stream.load()?));
                }
                b"AI_A" => {
                    stream.expect(33u32)?;
                    this.ai_packages
                        .get_or_insert_with(default)
                        .push(AiPackage::Activate(stream.load()?));
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

impl Save for Npc {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if let Some(value) = &self.mesh {
            stream.save(b"MODL")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // RNAM
        if let Some(value) = &self.race {
            stream.save(b"RNAM")?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.class {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        // ANAM
        if let Some(value) = &self.faction {
            stream.save(b"ANAM")?;
            stream.save(value)?;
        }
        // BNAM
        if let Some(value) = &self.head {
            stream.save(b"BNAM")?;
            stream.save(value)?;
        }
        // KNAM
        if let Some(value) = &self.hair {
            stream.save(b"KNAM")?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // NPDT
        if let Some(value) = &self.data {
            stream.save(b"NPDT")?;
            stream.save(value)?;
        }
        // FLAG
        if let Some(value) = &self.npc_flags {
            stream.save(b"FLAG")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // NPCO
        for value in self.inventory.iter().flatten() {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(value)?;
        }
        // NPCS
        for value in self.spells.iter().flatten() {
            stream.save(b"NPCS")?;
            stream.save(value)?;
        }
        // AIDT
        if let Some(value) = &self.ai_data {
            stream.save(b"AIDT")?;
            stream.save(&12u32)?;
            stream.save(value)?;
        }
        // DODT
        for value in self.travel_destinations.iter().flatten() {
            stream.save(b"DODT")?;
            stream.save(&24u32)?;
            stream.save(value)?;
        }
        //
        for value in self.ai_packages.iter().flatten() {
            match value {
                AiPackage::Travel(package) => {
                    // AI_T
                    stream.save(b"AI_T")?;
                    stream.save(&16u32)?;
                    stream.save(package)?;
                }
                AiPackage::Wander(package) => {
                    // AI_W
                    stream.save(b"AI_W")?;
                    stream.save(&14u32)?;
                    stream.save(package)?;
                }
                AiPackage::Escort(package) => {
                    // AI_E
                    stream.save(b"AI_E")?;
                    stream.save(&48u32)?;
                    stream.save(package)?;
                }
                AiPackage::Follow(package) => {
                    // AI_F
                    stream.save(b"AI_F")?;
                    stream.save(&48u32)?;
                    stream.save(package)?;
                }
                AiPackage::Activate(package) => {
                    // AI_A
                    stream.save(b"AI_A")?;
                    stream.save(&33u32)?;
                    stream.save(package)?;
                }
            }
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

impl Load for NpcData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self::default();

        let len: u32 = stream.load()?;
        match len {
            52 => {
                this.level = stream.load()?;
                this.stats = Some(stream.load()?);
                this.disposition = stream.load()?;
                this.reputation = stream.load()?;
                this.rank = stream.load()?;
                stream.skip(1)?; // padding
                this.gold = stream.load()?;
            }
            12 => {
                // auto-calc
                this.level = stream.load()?;
                this.disposition = stream.load()?;
                this.reputation = stream.load()?;
                this.rank = stream.load()?;
                stream.skip(3)?; // padding
                this.gold = stream.load()?;
            }
            _ => Reader::error(format!("Unexpected length ({}) for NPC_::NPDT", len))?,
        }

        Ok(this)
    }
}

impl Save for NpcData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        if let Some(stats) = &self.stats {
            stream.save(&52u32)?;
            stream.save(&self.level)?;
            stream.save(stats)?;
            stream.save(&self.disposition)?;
            stream.save(&self.reputation)?;
            stream.save(&self.rank)?;
        } else {
            // auto-calc
            stream.save(&12u32)?;
            stream.save(&self.level)?;
            stream.save(&self.disposition)?;
            stream.save(&self.reputation)?;
            stream.save(&self.rank)?;
            stream.save(&[0u8; 2])?; // padding
        }
        stream.save(&[0u8; 1])?; // padding
        stream.save(&self.gold)?;
        Ok(())
    }
}

impl Load for NpcStats {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let attributes = stream.load()?;
        let skills = stream.load()?;
        stream.skip(1)?; // padding
        let health = stream.load()?;
        let magicka = stream.load()?;
        let fatigue = stream.load()?;
        Ok(Self {
            attributes,
            skills,
            health,
            magicka,
            fatigue,
        })
    }
}

impl Save for NpcStats {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.attributes)?;
        stream.save(&self.skills)?;
        stream.save(&[0u8; 1])?; // padding
        stream.save(&self.health)?;
        stream.save(&self.magicka)?;
        stream.save(&self.fatigue)?;
        Ok(())
    }
}
