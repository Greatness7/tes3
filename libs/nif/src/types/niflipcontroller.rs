// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiFlipController {
    pub base: NiTimeController,
    pub affected_map: u32, // TODO enum
    pub flip_start_time: f32,
    pub secs_per_frame: f32,
    pub textures: Vec<NiLink<NiSourceTexture>>,
}

impl Load for NiFlipController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let affected_map = stream.load()?;
        let flip_start_time = stream.load()?;
        let secs_per_frame = stream.load()?;
        let textures = stream.load()?;
        Ok(Self {
            base,
            affected_map,
            flip_start_time,
            secs_per_frame,
            textures,
        })
    }
}

impl Save for NiFlipController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.affected_map)?;
        stream.save(&self.flip_start_time)?;
        stream.save(&self.secs_per_frame)?;
        stream.save(&self.textures)?;
        Ok(())
    }
}
