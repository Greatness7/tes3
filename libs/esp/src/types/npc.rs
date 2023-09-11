// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Npc {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub script: String,
    pub mesh: String,
    pub inventory: Vec<(i32, FixedString<32>)>,
    pub spells: Vec<String>,
    pub ai_data: AiData,
    pub ai_packages: Vec<AiPackage>,
    pub travel_destinations: Vec<TravelDestination>,
    pub race: String,
    pub class: String,
    pub faction: String,
    pub head: String,
    pub hair: String,
    pub npc_flags: NpcFlags,
    pub blood_type: u8,
    pub data: NpcData,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NpcData {
    pub level: i16,
    pub stats: Option<NpcStats>, // data here is garbage if (npc_flags) autocalc is not set
    pub disposition: i8,
    pub reputation: i8,
    pub rank: i8,
    pub gold: u32,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
                    this.mesh = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"RNAM" => {
                    this.race = stream.load()?;
                }
                b"CNAM" => {
                    this.class = stream.load()?;
                }
                b"ANAM" => {
                    this.faction = stream.load()?;
                }
                b"BNAM" => {
                    this.head = stream.load()?;
                }
                b"KNAM" => {
                    this.hair = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"NPDT" => {
                    this.data = stream.load()?;
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    let flags = stream.load()?;
                    (this.npc_flags, this.blood_type) = unpack_flags(flags);
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

impl Save for Npc {
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
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // RNAM
        if !self.race.is_empty() {
            stream.save(b"RNAM")?;
            stream.save(&self.race)?;
        }
        // CNAM
        if !self.class.is_empty() {
            stream.save(b"CNAM")?;
            stream.save(&self.class)?;
        }
        // ANAM
        if !self.faction.is_empty() {
            stream.save(b"ANAM")?;
            stream.save(&self.faction)?;
        }
        // BNAM
        if !self.head.is_empty() {
            stream.save(b"BNAM")?;
            stream.save(&self.head)?;
        }
        // KNAM
        if !self.hair.is_empty() {
            stream.save(b"KNAM")?;
            stream.save(&self.hair)?;
        }
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // NPDT
        stream.save(b"NPDT")?;
        stream.save(&self.data)?;
        // FLAG
        stream.save(b"FLAG")?;
        stream.save(&4u32)?;
        stream.save(&pack_flags(self.npc_flags, self.blood_type))?;
        // NPCO
        for value in &self.inventory {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(value)?;
        }
        // NPCS
        for value in &self.spells {
            stream.save(b"NPCS")?;
            stream.save(&32u32)?;
            stream.save::<FixedString<32>>(value.as_ref())?;
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

impl Load for NpcData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

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
            _ => Reader::error(format!("Unexpected length ({len}) for NPC_::NPDT"))?,
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

#[allow(clippy::cast_possible_truncation)]
const fn unpack_flags(flags: u32) -> (NpcFlags, u8) {
    // Note: drops unknown flags, may be `.ess` incompatible.
    let npc_flags = NpcFlags::from_bits_truncate(flags as u8);
    // Blood types start at the 10th bit and are 3 bits long.
    let blood_type = ((flags >> 10) & 0b111) as u8;
    (npc_flags, blood_type)
}

#[allow(clippy::cast_lossless)]
const fn pack_flags(npc_flags: NpcFlags, blood_type: u8) -> u32 {
    let flags = npc_flags.bits() as u32;
    let blood_type = blood_type as u32;
    flags | ((blood_type & 0b111) << 10)
}
