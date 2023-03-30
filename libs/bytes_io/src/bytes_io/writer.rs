// rust std imports
use std::borrow::Cow;
use std::io::{self, Write};

// external imports
use encoding_rs::{Encoding, WINDOWS_1252};
use hashbrown::HashMap;
use memchr::memchr;
use smart_default::SmartDefault;

// internal imports
use crate::Save;

#[derive(Debug, SmartDefault)]
pub struct Writer {
    pub cursor: io::Cursor<Vec<u8>>,
    pub context: HashMap<u64, u64>,
    #[default(WINDOWS_1252)]
    pub encoding: &'static Encoding,
}

impl Writer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            cursor: io::Cursor::new(bytes),
            ..Default::default()
        }
    }

    pub fn error<M>(message: M) -> io::Result<()>
    where
        M: Into<Cow<'static, str>>,
    {
        Err(io::Error::new(io::ErrorKind::InvalidData, message.into()))
    }

    pub fn save<S>(&mut self, value: &S) -> io::Result<()>
    where
        S: Save,
    {
        value.save(self)
    }

    pub fn save_as<T, S>(&mut self, value: T) -> io::Result<()>
    where
        S: Save + TryFrom<T>,
    {
        S::try_from(value).map_or_else(
            |_| Self::error("Invalid Save Conversion"), //
            |value| value.save(self),
        )
    }

    pub fn save_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.write_all(bytes)
    }

    pub fn save_string(&mut self, value: &str) -> io::Result<()> {
        if value.is_empty() {
            // save the string size
            self.save(&1u32)?;
            // save null terminator
            self.save(&0u8)?;
            return Ok(());
        }

        if let (bytes, _, false) = self.encoding.encode(value) {
            // scan for null terminator
            if let Some(index) = memchr(0, &bytes) {
                // save the string size
                self.save_as::<_, u32>(index)?;
                // save the string data
                self.save_bytes(&bytes[..index])?;
            } else {
                // save the string size
                self.save_as::<_, u32>(bytes.len() + 1)?;
                // save the string data
                self.save_bytes(&bytes)?;
                // save null terminator
                self.save(&0u8)?;
            }

            return Ok(());
        }

        Err(io::Error::new(io::ErrorKind::InvalidData, format!("encode error: {value}")))
    }

    pub fn save_string_without_null_terminator(&mut self, value: &str) -> io::Result<()> {
        let text = self.encode(value)?;
        self.save_as::<_, u32>(text.len())?;
        self.save_bytes(&text)?;
        Ok(())
    }

    pub fn encode<'a>(&self, str: &'a str) -> io::Result<Cow<'a, [u8]>> {
        if let (bytes, _, false) = self.encoding.encode(str) {
            Ok(match memchr(0, &bytes) {
                None => bytes,
                Some(i) => match bytes {
                    Cow::Borrowed(s) => s[..i].into(),
                    Cow::Owned(mut s) => {
                        s.truncate(i);
                        s.into()
                    }
                },
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("encode error: {str}")))
        }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.cursor.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.cursor.flush()
    }
}

#[cfg(feature = "nalgebra")]
const _: () = {
    use bytemuck::{cast_slice, Pod};
    use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, OMatrix, Scalar};

    impl Writer {
        pub fn save_matrix<S, R, C>(&mut self, matrix: &OMatrix<S, R, C>) -> io::Result<()>
        where
            S: Scalar + Pod,
            R: Dim,
            C: Dim,
            DefaultAllocator: Allocator<S, R, C>,
        {
            self.save_bytes(cast_slice(matrix.as_slice()))
        }
    }
};
