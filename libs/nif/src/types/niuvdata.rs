// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiUVData {
    pub base: NiObject,
    pub u_offset_data: NiFloatData,
    pub v_offset_data: NiFloatData,
    pub u_tiling_data: NiFloatData,
    pub v_tiling_data: NiFloatData,
}

impl Load for NiUVData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let u_offset_data = stream.load()?;
        let v_offset_data = stream.load()?;
        let u_tiling_data = stream.load()?;
        let v_tiling_data = stream.load()?;
        Ok(Self {
            base,
            u_offset_data,
            v_offset_data,
            u_tiling_data,
            v_tiling_data,
        })
    }
}

impl Save for NiUVData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.u_offset_data)?;
        stream.save(&self.v_offset_data)?;
        stream.save(&self.u_tiling_data)?;
        stream.save(&self.v_tiling_data)?;
        Ok(())
    }
}
