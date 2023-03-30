// external imports
use nalgebra::{dvector, Dyn, OMatrix, OVector, U3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiPixelData {
    pub base: NiObject,
    pub pixel_format: NiPixelFormat,
    pub palette: NiLink<NiPalette>,
    pub pixel_stride: u32,
    #[default(Empty::empty())]
    pub mipmaps: OMatrix<u32, U3, Dyn>,
    #[default(dvector![])]
    pub pixel_data: OVector<u8, Dyn>,
}

impl Load for NiPixelData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let pixel_format = stream.load()?;
        let palette = stream.load()?;
        let num_mipmap_levels = stream.load_as::<u32, _>()?;
        let pixel_stride = stream.load()?;
        let mipmaps = stream.load_matrix(3, num_mipmap_levels)?;
        let num_pixel_data = stream.load_as::<u32, _>()?;
        let pixel_data = stream.load_matrix(num_pixel_data, 1)?;
        Ok(Self {
            base,
            pixel_format,
            palette,
            pixel_stride,
            mipmaps,
            pixel_data,
        })
    }
}

impl Save for NiPixelData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.pixel_format)?;
        stream.save(&self.palette)?;
        stream.save_as::<_, u32>(self.mipmaps.ncols())?;
        stream.save(&self.pixel_stride)?;
        stream.save_matrix(&self.mipmaps)?;
        stream.save_as::<_, u32>(self.pixel_data.len())?;
        stream.save_matrix(&self.pixel_data)?;
        Ok(())
    }
}
