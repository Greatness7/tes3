// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct MagicEffect {
    pub flags: ObjectFlags,
    pub effect_id: EffectId,
    pub data: Option<MagicEffectData>,
    pub icon: Option<String>,
    pub texture: Option<String>,
    pub bolt_sound: Option<String>,
    pub cast_sound: Option<String>,
    pub hit_sound: Option<String>,
    pub area_sound: Option<String>,
    pub cast_visual: Option<String>,
    pub bolt_visual: Option<String>,
    pub hit_visual: Option<String>,
    pub area_visual: Option<String>,
    pub description: Option<String>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, LoadSave, Clone, Debug, Default, PartialEq)]
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
                    this.data = Some(stream.load()?);
                }
                b"ITEX" => {
                    this.icon = Some(stream.load()?);
                }
                b"PTEX" => {
                    this.texture = Some(stream.load()?);
                }
                b"BSND" => {
                    this.bolt_sound = Some(stream.load()?);
                }
                b"CSND" => {
                    this.cast_sound = Some(stream.load()?);
                }
                b"HSND" => {
                    this.hit_sound = Some(stream.load()?);
                }
                b"ASND" => {
                    this.area_sound = Some(stream.load()?);
                }
                b"CVFX" => {
                    this.cast_visual = Some(stream.load()?);
                }
                b"BVFX" => {
                    this.bolt_visual = Some(stream.load()?);
                }
                b"HVFX" => {
                    this.hit_visual = Some(stream.load()?);
                }
                b"AVFX" => {
                    this.area_visual = Some(stream.load()?);
                }
                b"DESC" => {
                    this.description = Some(stream.load()?);
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
        if let Some(value) = &self.data {
            stream.save(b"MEDT")?;
            stream.save(&36u32)?;
            stream.save(value)?;
        }
        // ITEX
        if let Some(value) = &self.icon {
            stream.save(b"ITEX")?;
            stream.save(value)?;
        }
        // PTEX
        if let Some(value) = &self.texture {
            stream.save(b"PTEX")?;
            stream.save(value)?;
        }
        // BSND
        if let Some(value) = &self.bolt_sound {
            stream.save(b"BSND")?;
            stream.save(value)?;
        }
        // CSND
        if let Some(value) = &self.cast_sound {
            stream.save(b"CSND")?;
            stream.save(value)?;
        }
        // HSND
        if let Some(value) = &self.hit_sound {
            stream.save(b"HSND")?;
            stream.save(value)?;
        }
        // ASND
        if let Some(value) = &self.area_sound {
            stream.save(b"ASND")?;
            stream.save(value)?;
        }
        // CVFX
        if let Some(value) = &self.cast_visual {
            stream.save(b"CVFX")?;
            stream.save(value)?;
        }
        // BVFX
        if let Some(value) = &self.bolt_visual {
            stream.save(b"BVFX")?;
            stream.save(value)?;
        }
        // HVFX
        if let Some(value) = &self.hit_visual {
            stream.save(b"HVFX")?;
            stream.save(value)?;
        }
        // AVFX
        if let Some(value) = &self.area_visual {
            stream.save(b"AVFX")?;
            stream.save(value)?;
        }
        // DESC
        if let Some(value) = &self.description {
            stream.save(b"DESC")?;
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
