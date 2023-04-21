#![allow(unused_variables)]

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct NiObject;

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
