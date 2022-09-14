// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Weapon {
    pub flags: ObjectFlags,
    pub id: String,
    pub data: Option<WeaponData>,
    pub mesh: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub script: Option<String>,
    pub enchanting: Option<String>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct WeaponData {
    pub weight: f32,
    pub value: u32,
    pub kind: WeaponType,
    pub health: u16,
    pub speed: f32,
    pub reach: f32,
    pub enchantment: u16,
    pub chop_min: u8,
    pub chop_max: u8,
    pub slash_min: u8,
    pub slash_max: u8,
    pub thrust_min: u8,
    pub thrust_max: u8,
    pub flags: u32,
}

impl Load for Weapon {
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
                b"WPDT" => {
                    stream.expect(32u32)?;
                    this.data = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"ITEX" => {
                    this.icon = Some(stream.load()?);
                }
                b"ENAM" => {
                    this.enchanting = Some(stream.load()?);
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

impl Save for Weapon {
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
        // WPDT
        if let Some(value) = &self.data {
            stream.save(b"WPDT")?;
            stream.save(&32u32)?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // ITEX
        if let Some(value) = &self.icon {
            stream.save(b"ITEX")?;
            stream.save(value)?;
        }
        // ENAM
        if let Some(value) = &self.enchanting {
            stream.save(b"ENAM")?;
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
