// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiPathController {
    pub base: NiTimeController,
    pub bank_direction: BankDirection,
    pub max_bank_angle: f32,
    pub smoothing: f32,
    pub follow_axis: u16,
    pub data: NiLink<NiPosData>,
    pub percentage_data: NiLink<NiFloatData>,
}

impl Load for NiPathController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let bank_direction = stream.load()?;
        let max_bank_angle = stream.load()?;
        let smoothing = stream.load()?;
        let follow_axis = stream.load()?;
        let data = stream.load()?;
        let percentage_data = stream.load()?;
        Ok(Self {
            base,
            bank_direction,
            max_bank_angle,
            smoothing,
            follow_axis,
            data,
            percentage_data,
        })
    }
}

impl Save for NiPathController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.bank_direction)?;
        stream.save(&self.max_bank_angle)?;
        stream.save(&self.smoothing)?;
        stream.save(&self.follow_axis)?;
        stream.save(&self.data)?;
        stream.save(&self.percentage_data)?;
        Ok(())
    }
}
