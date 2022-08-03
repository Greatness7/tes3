// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct Creature {
    pub flags: ObjectFlags,
    pub id: String,
    pub data: Option<CreatureData>,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub script: Option<String>,
    pub sound: Option<String>,
    pub creature_flags: Option<u32>,
    pub scale: Option<f32>,
    pub inventory: Option<Vec<(i32, FixedString<32>)>>,
    pub spells: Option<Vec<String>>,
    pub ai_data: Option<AiData>,
    pub ai_packages: Option<Vec<AiPackage>>,
    pub travel_destinations: Option<Vec<TravelDestination>>,
}

#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct CreatureData {
    pub kind: u32,
    pub level: u32,
    pub strength: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub agility: u32,
    pub speed: u32,
    pub endurance: u32,
    pub personality: u32,
    pub luck: u32,
    pub health: u32,
    pub magicka: u32,
    pub fatigue: u32,
    pub soul_points: u32,
    pub combat: u32,
    pub magic: u32,
    pub steath: u32,
    pub attack1: (u32, u32),
    pub attack2: (u32, u32),
    pub attack3: (u32, u32),
    pub gold: u32,
}

impl Load for Creature {
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
                b"CNAM" => {
                    this.sound = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"NPDT" => {
                    stream.expect(96u32)?;
                    this.data = Some(stream.load()?);
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.creature_flags = Some(stream.load()?);
                }
                b"XSCL" => {
                    stream.expect(4u32)?;
                    this.scale = Some(stream.load()?);
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

impl Save for Creature {
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
        // CNAM
        if let Some(value) = &self.sound {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
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
            stream.save(&96u32)?;
            stream.save(value)?;
        }
        // FLAG
        if let Some(value) = &self.creature_flags {
            stream.save(b"FLAG")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // XSCL
        if let Some(value) = &self.scale {
            stream.save(b"XSCL")?;
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
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

impl Load for TravelDestination {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let translation = stream.load()?;
        let rotation = stream.load()?;
        let cell = stream.expect(*b"DNAM").and_then(|_| stream.load()).ok();
        Ok(Self {
            translation,
            rotation,
            cell,
        })
    }
}

impl Save for TravelDestination {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.translation)?;
        stream.save(&self.rotation)?;
        if let Some(value) = &self.cell {
            stream.save(b"DNAM")?;
            stream.save(value)?;
        }
        Ok(())
    }
}
