// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reference {
    pub mast_index: u32,
    pub refr_index: u32,
    pub id: String,
    pub temporary: bool,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: Option<f32>,
    pub moved_cell: Option<(i32, i32)>,
    pub owner: Option<String>,
    pub owner_global: Option<String>,
    pub owner_faction: Option<String>,
    pub owner_faction_rank: Option<u32>,
    pub charge_left: Option<u32>,
    pub health_left: Option<i32>,
    pub object_count: Option<u32>,
    pub destination: Option<TravelDestination>,
    pub lock_level: Option<i32>,
    pub key: Option<String>,
    pub trap: Option<String>,
    pub soul: Option<String>,
    pub blocked: Option<u8>,
    pub deleted: Option<bool>,
}

impl Load for Reference {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"UNAM" => {
                    stream.expect(1u32)?;
                    this.blocked = Some(stream.load()?);
                }
                b"XSCL" => {
                    stream.expect(4u32)?;
                    this.scale = Some(stream.load()?);
                }
                b"ANAM" => {
                    this.owner = Some(stream.load()?);
                }
                b"BNAM" => {
                    this.owner_global = Some(stream.load()?);
                }
                b"CNAM" => {
                    this.owner_faction = Some(stream.load()?);
                }
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.owner_faction_rank = Some(stream.load()?);
                }
                b"XSOL" => {
                    this.soul = Some(stream.load()?);
                }
                b"XCHG" => {
                    stream.expect(4u32)?;
                    this.charge_left = Some(stream.load()?);
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    this.health_left = Some(stream.load()?);
                }
                b"NAM9" => {
                    stream.expect(4u32)?;
                    this.object_count = Some(stream.load()?);
                }
                b"DODT" => {
                    stream.expect(24u32)?;
                    this.destination = Some(stream.load()?);
                }
                b"FLTV" => {
                    stream.expect(4u32)?;
                    this.lock_level = Some(stream.load()?);
                }
                b"KNAM" => {
                    this.key = Some(stream.load()?);
                }
                b"TNAM" => {
                    this.trap = Some(stream.load()?);
                }
                b"DATA" => {
                    stream.expect(24u32)?;
                    this.translation = stream.load()?;
                    this.rotation = stream.load()?;
                    break;
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.deleted = Some(true);
                    break;
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: REFR::{}", tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Reference {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // UNAM
        if let Some(value) = &self.blocked {
            stream.save(b"UNAM")?;
            stream.save(&1u32)?;
            stream.save(value)?;
        }
        // XSCL
        if let Some(value) = &self.scale {
            let scale = value.clamp(0.5, 2.0);
            let scale_is_default = (scale - 1.0).abs() < 1e-6;
            if !scale_is_default || (self.mast_index != 0) {
                stream.save(b"XSCL")?;
                stream.save(&4u32)?;
                stream.save(&scale)?;
            }
        }
        // ANAM
        if let Some(value) = &self.owner {
            stream.save(b"ANAM")?;
            stream.save(value)?;
        }
        // BNAM
        if let Some(value) = &self.owner_global {
            stream.save(b"BNAM")?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.owner_faction {
            stream.save(b"CNAM")?;
            stream.save(value)?;
        }
        // INDX
        if let Some(value) = &self.owner_faction_rank {
            stream.save(b"INDX")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // XSOL
        if let Some(value) = &self.soul {
            stream.save(b"XSOL")?;
            stream.save(value)?;
        }
        // XCHG
        if let Some(value) = &self.charge_left {
            stream.save(b"XCHG")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // INTV
        if let Some(value) = &self.health_left {
            stream.save(b"INTV")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // NAM9
        if let Some(value) = &self.object_count {
            let object_count_is_default = *value == 1;
            if !object_count_is_default || (self.mast_index != 0) {
                stream.save(b"NAM9")?;
                stream.save(&4u32)?;
                stream.save(value)?;
            }
        }
        // DODT
        if let Some(value) = &self.destination {
            stream.save(b"DODT")?;
            stream.save(&24u32)?;
            stream.save(value)?;
        }
        // FLTV
        if let Some(value) = &self.lock_level {
            stream.save(b"FLTV")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // KNAM
        if let Some(value) = &self.key {
            stream.save(b"KNAM")?;
            stream.save(value)?;
        }
        // TNAM
        if let Some(value) = &self.trap {
            stream.save(b"TNAM")?;
            stream.save(value)?;
        }
        // DELE
        if self.deleted.is_some() {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        } else {
            // DATA
            stream.save(b"DATA")?;
            stream.save(&24u32)?;
            stream.save(&self.translation)?;
            stream.save(&self.rotation)?;
        }
        Ok(())
    }
}

impl Reference {
    pub const fn persistent(&self) -> bool {
        // Moved references are always persistent.
        if self.moved_cell.is_some() {
            return true;
        }

        // Doors with destinations are always persistent.
        if self.destination.is_some() {
            return true;
        }

        // Note:
        //  NPCs and creatures should also always be persistent.
        //  We do not have the information to enforce that here.

        // For everything else trust the flag.
        !self.temporary
    }
}
