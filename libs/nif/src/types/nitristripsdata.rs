// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTriStripsData {
    pub base: NiTriBasedGeomData,
    pub num_triangles: u16,
    pub strip_lengths: Vec<u16>,
    pub strips: Vec<u16>,
}

impl Load for NiTriStripsData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_triangles = stream.load()?;
        let num_strip_lengths: u16 = stream.load()?;
        let strip_lengths: Vec<u16> = stream.load_vec(num_strip_lengths)?;
        let strip_lengths_sum: usize = strip_lengths.iter().map(|n| *n as usize).sum();
        let strips = stream.load_vec(strip_lengths_sum)?;
        Ok(Self {
            base,
            num_triangles,
            strip_lengths,
            strips,
        })
    }
}

impl Save for NiTriStripsData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.num_triangles)?;
        stream.save_as::<u16>(self.strip_lengths.len())?;
        stream.save_vec(&self.strip_lengths)?;
        stream.save_vec(&self.strips)?;
        Ok(())
    }
}
