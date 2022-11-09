// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MagicEffect {
    pub flags: ObjectFlags,
    pub effect_id: EffectId,
    pub data: MagicEffectData,
    pub icon: String,
    pub texture: String,
    pub bolt_sound: String,
    pub cast_sound: String,
    pub hit_sound: String,
    pub area_sound: String,
    pub cast_visual: String,
    pub bolt_visual: String,
    pub hit_visual: String,
    pub area_visual: String,
    pub description: String,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct MagicEffectData {
    pub school: EffectSchool,
    pub base_cost: f32,
    pub flags: u32,
    pub color: (i32, i32, i32),
    pub speed: f32,
    pub size: f32,
    pub size_cap: f32,
}

impl Load for MagicEffect {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"INDX" => {
                    stream.expect(4u32)?;
                    this.effect_id = stream.load()?;
                }
                b"MEDT" => {
                    stream.expect(36u32)?;
                    this.data = stream.load()?;
                }
                b"ITEX" => {
                    this.icon = stream.load()?;
                }
                b"PTEX" => {
                    this.texture = stream.load()?;
                }
                b"BSND" => {
                    this.bolt_sound = stream.load()?;
                }
                b"CSND" => {
                    this.cast_sound = stream.load()?;
                }
                b"HSND" => {
                    this.hit_sound = stream.load()?;
                }
                b"ASND" => {
                    this.area_sound = stream.load()?;
                }
                b"CVFX" => {
                    this.cast_visual = stream.load()?;
                }
                b"BVFX" => {
                    this.bolt_visual = stream.load()?;
                }
                b"HVFX" => {
                    this.hit_visual = stream.load()?;
                }
                b"AVFX" => {
                    this.area_visual = stream.load()?;
                }
                b"DESC" => {
                    this.description = stream.load()?;
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

impl Save for MagicEffect {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // INDX
        stream.save(b"INDX")?;
        stream.save(&4u32)?;
        stream.save(&self.effect_id)?;
        // MEDT
        stream.save(b"MEDT")?;
        stream.save(&36u32)?;
        stream.save(&self.data)?;
        // ITEX
        if !self.icon.is_empty() {
            stream.save(b"ITEX")?;
            stream.save(&self.icon)?;
        }
        // PTEX
        if !self.texture.is_empty() {
            stream.save(b"PTEX")?;
            stream.save(&self.texture)?;
        }
        // BSND
        if !self.bolt_sound.is_empty() {
            stream.save(b"BSND")?;
            stream.save(&self.bolt_sound)?;
        }
        // CSND
        if !self.cast_sound.is_empty() {
            stream.save(b"CSND")?;
            stream.save(&self.cast_sound)?;
        }
        // HSND
        if !self.hit_sound.is_empty() {
            stream.save(b"HSND")?;
            stream.save(&self.hit_sound)?;
        }
        // ASND
        if !self.area_sound.is_empty() {
            stream.save(b"ASND")?;
            stream.save(&self.area_sound)?;
        }
        // CVFX
        if !self.cast_visual.is_empty() {
            stream.save(b"CVFX")?;
            stream.save(&self.cast_visual)?;
        }
        // BVFX
        if !self.bolt_visual.is_empty() {
            stream.save(b"BVFX")?;
            stream.save(&self.bolt_visual)?;
        }
        // HVFX
        if !self.hit_visual.is_empty() {
            stream.save(b"HVFX")?;
            stream.save(&self.hit_visual)?;
        }
        // AVFX
        if !self.area_visual.is_empty() {
            stream.save(b"AVFX")?;
            stream.save(&self.area_visual)?;
        }
        // DESC
        if !self.description.is_empty() {
            stream.save(b"DESC")?;
            stream.save(&self.description)?;
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
