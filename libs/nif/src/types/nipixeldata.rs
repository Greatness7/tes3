// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Eq, PartialEq, SmartDefault)]
pub struct NiPixelData {
    pub base: NiObject,
    pub pixel_format: NiPixelFormat,
    pub palette: NiLink<NiPalette>,
    pub pixel_stride: u32,
    pub mipmaps: Vec<[u32; 3]>, // [width, height, offset]
    pub pixel_data: Vec<u8>,
}

impl Load for NiPixelData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let pixel_format = stream.load()?;
        let palette = stream.load()?;
        let num_mipmap_levels: u32 = stream.load()?;
        let pixel_stride = stream.load()?;
        let mipmaps = stream.load_vec(num_mipmap_levels)?;
        let num_pixel_data: u32 = stream.load()?;
        let pixel_data = stream.load_vec(num_pixel_data)?;
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
        stream.save_as::<u32>(self.mipmaps.len())?;
        stream.save(&self.pixel_stride)?;
        stream.save_vec(&self.mipmaps)?;
        stream.save_as::<u32>(self.pixel_data.len())?;
        stream.save_vec(&self.pixel_data)?;
        Ok(())
    }
}
