// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct NiPixelFormat {
    pub base: NiObject,
    pub pixel_format: PixelFormat,
    pub color_masks: [u32; 4],
    pub bits_per_pixel: u32,
    pub compare_bits: [u8; 8],
}

impl Load for NiPixelFormat {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let pixel_format = stream.load()?;
        let color_masks = stream.load()?;
        let bits_per_pixel = stream.load()?;
        let compare_bits = stream.load()?;
        Ok(Self {
            base,
            pixel_format,
            color_masks,
            bits_per_pixel,
            compare_bits,
        })
    }
}

impl Save for NiPixelFormat {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.pixel_format)?;
        stream.save(&self.color_masks)?;
        stream.save(&self.bits_per_pixel)?;
        stream.save(&self.compare_bits)?;
        Ok(())
    }
}
