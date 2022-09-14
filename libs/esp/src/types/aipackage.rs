// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub enum AiPackage {
    #[default]
    Travel(AiTravelPackage),
    Wander(AiWanderPackage),
    Escort(AiEscortPackage),
    Follow(AiFollowPackage),
    Activate(AiActivatePackage),
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AiTravelPackage {
    pub location: [f32; 3],
    pub reset: u8,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct AiWanderPackage {
    pub distance: u16,
    pub duration: u16,
    pub game_hour: u8,
    pub idle2: u8,
    pub idle3: u8,
    pub idle4: u8,
    pub idle5: u8,
    pub idle6: u8,
    pub idle7: u8,
    pub idle8: u8,
    pub idle9: u8,
    pub reset: i8,
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AiEscortPackage {
    pub location: [f32; 3],
    pub duration: u16,
    pub target: FixedString<32>,
    pub reset: u8,
    pub cell: Option<String>,
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AiFollowPackage {
    pub location: [f32; 3],
    pub duration: u16,
    pub target: FixedString<32>,
    pub reset: u8,
    pub cell: Option<String>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct AiActivatePackage {
    pub target: FixedString<32>,
    pub reset: u8,
}

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TravelDestination {
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub cell: Option<String>,
}

impl Load for AiTravelPackage {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let location = stream.load()?;
        let reset = stream.load()?;
        stream.skip(3)?; // padding
        Ok(Self { location, reset })
    }
}

impl Save for AiTravelPackage {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.location)?;
        stream.save(&self.reset)?;
        stream.save(&[0u8; 3])?; // padding
        Ok(())
    }
}

impl Load for AiEscortPackage {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let location = stream.load()?;
        let duration = stream.load()?;
        let target = stream.load()?;
        let reset = stream.load()?;
        stream.skip(1)?; // padding
        let cell = stream.expect(*b"CNDT").and_then(|_| stream.load()).ok();
        Ok(Self {
            location,
            duration,
            target,
            reset,
            cell,
        })
    }
}

impl Save for AiEscortPackage {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.location)?;
        stream.save(&self.duration)?;
        stream.save(&self.target)?;
        stream.save(&self.reset)?;
        stream.save(&[0u8; 1])?; // padding
        if let Some(value) = &self.cell {
            stream.save(b"CNDT")?;
            stream.save(value)?;
        }
        Ok(())
    }
}

impl Load for AiFollowPackage {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let location = stream.load()?;
        let duration = stream.load()?;
        let target = stream.load()?;
        let reset = stream.load()?;
        stream.skip(1)?; // padding
        let cell = stream.expect(*b"CNDT").and_then(|_| stream.load()).ok();
        Ok(Self {
            location,
            duration,
            target,
            reset,
            cell,
        })
    }
}

impl Save for AiFollowPackage {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.location)?;
        stream.save(&self.duration)?;
        stream.save(&self.target)?;
        stream.save(&self.reset)?;
        stream.save(&[0u8; 1])?; // padding
        if let Some(value) = &self.cell {
            stream.save(b"CNDT")?;
            stream.save(value)?;
        }
        Ok(())
    }
}
