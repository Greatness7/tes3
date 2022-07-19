// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSortAdjustNode {
    pub base: NiNode,
    pub sorting_mode: SortingMode,
    pub sub_sorter: NiLink<NiAccumulator>,
}

impl Load for NiSortAdjustNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let sorting_mode = stream.load()?;
        let sub_sorter = stream.load()?;
        Ok(Self {
            base,
            sorting_mode,
            sub_sorter,
        })
    }
}

impl Save for NiSortAdjustNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.sorting_mode)?;
        stream.save(&self.sub_sorter)?;
        Ok(())
    }
}
