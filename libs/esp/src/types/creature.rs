// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Creature {
    pub flags: ObjectFlags,
    pub id: String,
    pub data: CreatureData,
    pub name: String,
    pub mesh: String,
    pub script: String,
    pub sound: String,
    pub creature_flags: u32,
    pub scale: f32,
    pub inventory: Vec<(i32, FixedString<32>)>,
    pub spells: Vec<String>,
    pub ai_data: AiData,
    pub ai_packages: Vec<AiPackage>,
    pub travel_destinations: Vec<TravelDestination>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
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
                    this.mesh = stream.load()?;
                }
                b"CNAM" => {
                    this.sound = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"NPDT" => {
                    stream.expect(96u32)?;
                    this.data = stream.load()?;
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.creature_flags = stream.load()?;
                }
                b"XSCL" => {
                    stream.expect(4u32)?;
                    this.scale = stream.load()?;
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.push(stream.load()?);
                }
                b"NPCS" => {
                    this.spells.push(stream.load()?);
                }
                b"AIDT" => {
                    stream.expect(12u32)?;
                    this.ai_data = stream.load()?;
                }
                b"DODT" => {
                    stream.expect(24u32)?;
                    this.travel_destinations.push(stream.load()?);
                }
                b"AI_T" => {
                    stream.expect(16u32)?;
                    this.ai_packages.push(AiPackage::Travel(stream.load()?));
                }
                b"AI_W" => {
                    stream.expect(14u32)?;
                    this.ai_packages.push(AiPackage::Wander(stream.load()?));
                }
                b"AI_E" => {
                    stream.expect(48u32)?;
                    this.ai_packages.push(AiPackage::Escort(stream.load()?));
                }
                b"AI_F" => {
                    stream.expect(48u32)?;
                    this.ai_packages.push(AiPackage::Follow(stream.load()?));
                }
                b"AI_A" => {
                    stream.expect(33u32)?;
                    this.ai_packages.push(AiPackage::Activate(stream.load()?));
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
        if !self.mesh.is_empty() {
            stream.save(b"MODL")?;
            stream.save(&self.mesh)?;
        }
        // CNAM
        if !self.sound.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.sound)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // NPDT
        stream.save(b"NPDT")?;
        stream.save(&96u32)?;
        stream.save(&self.data)?;
        // FLAG
        stream.save(b"FLAG")?;
        stream.save(&4u32)?;
        stream.save(&self.creature_flags)?;
        // XSCL
        stream.save(b"XSCL")?;
        stream.save(&4u32)?;
        stream.save(&self.scale)?;
        // NPCO
        for value in &self.inventory {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(value)?;
        }
        // NPCS
        for value in &self.spells {
            stream.save(b"NPCS")?;
            stream.save(value)?;
        }
        // AIDT
        stream.save(b"AIDT")?;
        stream.save(&12u32)?;
        stream.save(&self.ai_data)?;
        // DODT
        for value in &self.travel_destinations {
            stream.save(b"DODT")?;
            stream.save(&24u32)?;
            stream.save(value)?;
        }
        //
        for value in &self.ai_packages {
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
