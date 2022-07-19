// external imports
use nalgebra::{Dynamic, OMatrix, U4};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiPalette {
    pub base: NiObject,
    pub has_alpha: bool,
    #[default(OMatrix::<u8, U4, Dynamic>::zeros(0))]
    pub palettes: OMatrix<u8, U4, Dynamic>,
}

impl Load for NiPalette {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let has_alpha = stream.load::<u8>()? != 0;
        let num_palettes = stream.load_as::<u32, _>()?;
        let palettes = stream.load_matrix(4, num_palettes)?;
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
        stream.save_as::<_, u8>(self.has_alpha)?;
        stream.save_as::<_, u32>(self.palettes.ncols())?;
        stream.save_matrix(&self.palettes)?;
        Ok(())
    }
}
