// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiBoundingVolume {
    pub bound_data: BoundData,
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub enum BoundData {
    #[default]
    NiBoxBV(NiBoxBV),
    NiSphereBV(NiSphereBV),
    NiUnionBV(NiUnionBV),
}

impl BoundData {
    const fn bound_type(&self) -> BoundType {
        match self {
            BoundData::NiBoxBV(_) => BoundType::Box,
            BoundData::NiSphereBV(_) => BoundType::Sphere,
            BoundData::NiUnionBV(_) => BoundType::Union,
        }
    }
}

impl Load for NiBoundingVolume {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let bound_type = stream.load()?;
        let bound_data = match bound_type {
            BoundType::Box => BoundData::NiBoxBV(stream.load()?),
            BoundType::Sphere => BoundData::NiSphereBV(stream.load()?),
            BoundType::Union => BoundData::NiUnionBV(stream.load()?),
            _ => Reader::error(format!("Invalid BoundType: {bound_type:?}"))?,
        };
        Ok(Self { bound_data })
    }
}

impl Save for NiBoundingVolume {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.bound_data.bound_type())?;
        match &self.bound_data {
            BoundData::NiBoxBV(data) => stream.save(data)?,
            BoundData::NiSphereBV(data) => stream.save(data)?,
            BoundData::NiUnionBV(data) => stream.save(data)?,
        }
        Ok(())
    }
}
