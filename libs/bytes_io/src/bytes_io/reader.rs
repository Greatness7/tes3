// rust std imports
use std::borrow::Cow;
use std::io::{self, Read};

// external imports
use encoding_rs::{Encoding, WINDOWS_1252};
use memchr::memchr;
use smart_default::SmartDefault;

// internal imports
use crate::Load;

#[derive(Debug, SmartDefault)]
pub struct Reader<'a> {
    pub cursor: io::Cursor<&'a [u8]>,
    #[default(WINDOWS_1252)]
    pub encoding: &'static Encoding,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: io::Cursor::new(bytes),
            ..Default::default()
        }
    }

    pub fn error<M, T>(message: M) -> io::Result<T>
    where
        M: Into<Cow<'static, str>>,
    {
        Err(io::Error::new(io::ErrorKind::InvalidData, message.into()))
    }

    pub fn load<L>(&mut self) -> io::Result<L>
    where
        L: Load,
    {
        L::load(self)
    }

    pub fn load_as<L, T>(&mut self) -> io::Result<T>
    where
        L: Load + TryInto<T>,
    {
        L::load(self)?
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid Load Conversion"))
    }

    pub fn load_bytes(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut bytes = vec![0; len];
        self.cursor.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    pub fn load_string<T>(&mut self, len: usize) -> io::Result<T>
    where
        for<'any> Cow<'any, str>: Into<T>,
    {
        if len == 0 {
            return Ok(Cow::from("").into());
        }

        // truncate at first null character
        let mut bytes = self.load_bytes(len)?;
        if let Some(index) = memchr(0, &bytes) {
            bytes.truncate(index);
        }

        if let (bytes, _, false) = self.encoding.decode(&bytes) {
            return Ok(bytes.into());
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("decode error: offset={}", self.cursor.position()),
        ))
    }

    pub fn expect<L>(&mut self, expected: L) -> io::Result<()>
    where
        L: Copy + Load + PartialEq,
    {
        let pos = self.cursor.position();
        let value: L = self.load()?;
        if value == expected {
            Ok(())
        } else {
            self.cursor.set_position(pos);
            Self::error("Unexpected Value")
        }
    }

    pub fn skip(&mut self, len: u32) -> io::Result<u64> {
        let old_pos = self.cursor.position();
        let new_pos = old_pos + u64::from(len);
        if new_pos <= (self.cursor.get_ref().len() as u64) {
            self.cursor.set_position(new_pos);
            Ok(new_pos)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Skip out of bounds"))
        }
    }
}

impl Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

#[cfg(feature = "nalgebra")]
const _: () = {
    use bytemuck::{cast_slice_mut, Pod};
    use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, OMatrix, Scalar};

    impl Reader<'_> {
        pub fn load_matrix<S, R, C>(&mut self, nrows: usize, ncols: usize) -> io::Result<OMatrix<S, R, C>>
        where
            S: Scalar + Pod,
            R: Dim,
            C: Dim,
            DefaultAllocator: Allocator<S, R, C>,
        {
            let mut this = OMatrix::repeat_generic(R::from_usize(nrows), C::from_usize(ncols), S::zeroed());
            self.cursor.read_exact(cast_slice_mut(this.as_mut_slice()))?;
            Ok(this)
        }
    }
};
