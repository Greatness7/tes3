// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiObjectNET {
    pub base: NiObject,
    pub name: String,
    pub extra_data: NiLink<NiExtraData>,
    pub controller: NiLink<NiTimeController>,
}

impl Load for NiObjectNET {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let name = stream.load()?;
        let extra_data = stream.load()?;
        let controller = stream.load()?;
        Ok(Self {
            base,
            name,
            extra_data,
            controller,
        })
    }
}

impl Save for NiObjectNET {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_string_without_null_terminator(&self.name)?;
        stream.save(&self.extra_data)?;
        stream.save(&self.controller)?;
        Ok(())
    }
}

impl NiObjectNET {
    pub fn extra_datas<'a>(&'a self, stream: &'a NiStream) -> impl Iterator<Item = &'a NiType> {
        let mut next = self.extra_data;
        std::iter::from_fn(move || {
            let object = stream.objects.get(next.key)?;
            let extra_data: &NiExtraData = object.try_into().ok()?;
            next = extra_data.next;
            Some(object)
        })
    }

    pub fn extra_datas_of_type<'a, T>(&'a self, stream: &'a NiStream) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.extra_datas(stream).filter_map(|object| object.try_into().ok())
    }
}
