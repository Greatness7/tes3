// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct Region {
    pub flags1: u32,
    pub flags2: u32,
    pub id: String,
    pub name: Option<String>,
    pub weather_chances: Option<WeatherChances>,
    pub sleep_creature: Option<String>,
    pub map_color: Option<[u8; 4]>,
    pub sounds: Option<Vec<(FixedString<32>, u8)>>,
    pub deleted: Option<u32>,
}

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct WeatherChances {
    pub clear: u8,
    pub cloudy: u8,
    pub foggy: u8,
    pub overcast: u8,
    pub rain: u8,
    pub thunder: u8,
    pub ash: u8,
    pub blight: u8,
    pub snow: u8,
    pub blizzard: u8,
}

impl Load for Region {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self {
            flags1: stream.load()?,
            flags2: stream.load()?,
            ..default()
        };

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"WEAT" => {
                    this.weather_chances = Some(stream.load()?);
                }
                b"BNAM" => {
                    this.sleep_creature = Some(stream.load()?);
                }
                b"CNAM" => {
                    stream.expect(4u32)?;
                    this.map_color = Some(stream.load()?);
                }
                b"SNAM" => {
                    stream.expect(33u32)?;
                    this.sounds.get_or_insert_with(default).push(stream.load()?);
                }
                b"DELE" => {
                    stream.expect(4u32)?;
                    this.deleted = Some(stream.load()?);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Region {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags1)?;
        stream.save(&self.flags2)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // WEAT
        if let Some(value) = &self.weather_chances {
            stream.save(b"WEAT")?;
            stream.save(value)?;
        }
        // BNAM
        if let Some(value) = &self.sleep_creature {
            stream.save(b"BNAM")?;
            stream.save(value)?;
        }
        // CNAM
        if let Some(value) = &self.map_color {
            stream.save(b"CNAM")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // SNAM
        for (sound, chance) in self.sounds.iter().flatten() {
            stream.save(b"SNAM")?;
            stream.save(&33u32)?;
            stream.save(sound)?;
            stream.save(chance)?;
        }
        // DELE
        if let Some(value) = &self.deleted {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        Ok(())
    }
}

impl Load for WeatherChances {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len: u32 = stream.load()?;
        assert!((len == 8 || len == 10), "Unexpected size ({}) for REGN::WEAT", len);
        let clear = stream.load()?;
        let cloudy = stream.load()?;
        let foggy = stream.load()?;
        let overcast = stream.load()?;
        let rain = stream.load()?;
        let thunder = stream.load()?;
        let ash = stream.load()?;
        let blight = stream.load()?;
        let snow = if len == 10 { stream.load()? } else { 0 };
        let blizzard = if len == 10 { stream.load()? } else { 0 };
        Ok(Self {
            clear,
            cloudy,
            foggy,
            overcast,
            rain,
            thunder,
            ash,
            blight,
            snow,
            blizzard,
        })
    }
}

impl Save for WeatherChances {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&10u32)?;
        stream.save(&self.clear)?;
        stream.save(&self.cloudy)?;
        stream.save(&self.foggy)?;
        stream.save(&self.overcast)?;
        stream.save(&self.rain)?;
        stream.save(&self.thunder)?;
        stream.save(&self.ash)?;
        stream.save(&self.blight)?;
        stream.save(&self.snow)?;
        stream.save(&self.blizzard)?;
        Ok(())
    }
}
