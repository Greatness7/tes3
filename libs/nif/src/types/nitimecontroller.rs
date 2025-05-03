// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTimeController {
    pub base: NiObject,
    pub next: NiLink<NiTimeController>,
    pub flags: u16,
    #[default(1.0)]
    pub frequency: f32,
    pub phase: f32,
    pub start_time: f32,
    pub stop_time: f32,
    pub target: NiLink<NiObjectNET>,
}

impl Load for NiTimeController {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let next = stream.load()?;
        let flags = stream.load()?;
        let frequency = stream.load()?;
        let phase = stream.load()?;
        let start_time = stream.load()?;
        let stop_time = stream.load()?;
        let target = stream.load()?;
        Ok(Self {
            base,
            next,
            flags,
            frequency,
            phase,
            start_time,
            stop_time,
            target,
        })
    }
}

impl Save for NiTimeController {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.next)?;
        stream.save(&self.flags)?;
        stream.save(&self.frequency)?;
        stream.save(&self.phase)?;
        stream.save(&self.start_time)?;
        stream.save(&self.stop_time)?;
        stream.save(&self.target)?;
        Ok(())
    }
}

impl NiTimeController {
    flag_props! {
        cycle_type @ (mask = 0x0006, pos = 1) -> CycleType,
        active @ (mask = 0x0008) -> bool,
    }
}
