// rust std imports
use std::borrow::Borrow;

// external imports
use bytemuck::TransparentWrapper;

// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(TransparentWrapper)]
#[derive(Clone, Debug, Default, Deref, DerefMut, Eq, From, Into, PartialEq)]
#[cfg_attr(feature = "serde", serde(transparent))]
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
                format!("Invalid string length ({} != {N}): {}", self.len(), self.as_str()),
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
