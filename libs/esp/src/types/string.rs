// rust std imports
use std::borrow::Borrow;

// external imports
use bytemuck::TransparentWrapper;
use derive_more::{Deref, DerefMut, From, Into};

// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(TransparentWrapper)]
#[derive(Deref, DerefMut, From, Into)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct FixedString<const N: usize>(pub String);

impl<const N: usize> Load for FixedString<N> {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok(Self(stream.load_string(N)?))
    }
}

impl<const N: usize> Save for FixedString<N> {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        let bytes = stream.encode(self)?;
        if bytes.len() > N {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid string length ({} != {}): {}", self.len(), N, self.as_str()),
            ));
        }
        stream.save_bytes(&bytes)?;
        stream.save_bytes(&[0u8; N][bytes.len()..])?; // padding
        Ok(())
    }
}

impl<const N: usize> Borrow<str> for FixedString<N> {
    fn borrow(&self) -> &str {
        self
    }
}

impl<const N: usize> AsRef<FixedString<N>> for String {
    fn as_ref(&self) -> &FixedString<N> {
        TransparentWrapper::wrap_ref(self)
    }
}
