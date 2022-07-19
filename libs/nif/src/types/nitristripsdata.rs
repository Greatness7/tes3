// external imports
use nalgebra::{dvector, Dynamic, OVector};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTriStripsData {
    pub base: NiTriBasedGeomData,
    pub num_triangles: u16,
    #[default(dvector![])]
    pub strip_lengths: OVector<u16, Dynamic>,
    #[default(dvector![])]
    pub strips: OVector<u16, Dynamic>,
}

impl Load for NiTriStripsData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_triangles = stream.load()?;
        let num_strip_lengths = stream.load_as::<u16, _>()?;
        let strip_lengths = stream.load_matrix(num_strip_lengths, 1)?;
        let strip_lengths_sum = OVector::sum(&strip_lengths) as usize;
        let strips = stream.load_matrix(strip_lengths_sum, 1)?;
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
        stream.save_as::<_, u16>(self.strip_lengths.len())?;
        if !self.strip_lengths.is_empty() {
            stream.save_matrix(&self.strip_lengths)?;
            stream.save_matrix(&self.strips)?;
        }
        Ok(())
    }
}
