// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NiObject;

impl NiObject {
    #[doc(hidden)]
    #[allow(clippy::unused_self)]
    pub const fn type_name(&self) -> &'static [u8] {
        b"NiObject"
    }
}

impl Load for NiObject {
    #![allow(unused_variables)]
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok(Self)
    }
}

impl Save for NiObject {
    #![allow(unused_variables)]
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        Ok(())
    }
}
