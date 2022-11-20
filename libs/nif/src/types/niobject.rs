#![allow(unused_variables)]

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NiObject;

impl NiObject {
    #[doc(hidden)]
    pub const fn type_name(&self) -> &'static [u8] {
        b"NiObject"
    }
}

impl Load for NiObject {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok(Self)
    }
}

impl Save for NiObject {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        Ok(())
    }
}
