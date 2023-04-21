// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiPalette {
    pub base: NiObject,
    pub has_alpha: bool,
    pub palettes: Vec<[u8; 4]>, // [r, g, b, a]
}

impl Load for NiPalette {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let has_alpha = stream.load::<u8>()? != 0;
        let num_palettes: u32 = stream.load()?;
        let palettes = stream.load_vec(num_palettes)?;
        Ok(Self {
            base,
            has_alpha,
            palettes,
        })
    }
}

impl Save for NiPalette {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<u8>(self.has_alpha)?;
        stream.save_as::<u32>(self.palettes.len())?;
        stream.save_vec(&self.palettes)?;
        Ok(())
    }
}
