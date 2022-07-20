// rust std imports
use std::io::{self, Read};

// external imports
use bstr::BString;
use bytemuck::{bytes_of_mut, Pod, Zeroable};
use copyless::BoxHelper;
use nalgebra::{allocator::Allocator, DefaultAllocator, DimName, OMatrix, Scalar};

// internal imports
use crate::bytes_io::{AsRepr, Reader};

pub trait Load: Sized {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self>;
}

impl Load for String {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len = stream.load::<u32>()? as usize;
        stream.load_string(len)
    }
}

impl Load for BString {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len = stream.load::<u32>()? as usize;
        Ok(stream.load_bytes(len)?.into())
    }
}

impl<L: Load> Load for Box<L> {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok(Box::alloc().init(stream.load()?))
    }
}

impl<L: Load> Load for Vec<L> {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len = stream.load::<u32>()?;
        (0..len).map(|_| stream.load()).collect()
    }
}

impl<S, R, C> Load for OMatrix<S, R, C>
where
    Self: Pod,
    S: Scalar,
    R: DimName,
    C: DimName,
    DefaultAllocator: Allocator<S, R, C>,
{
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this = Self::zeroed();
        stream.cursor.read_exact(bytes_of_mut(&mut this))?;
        Ok(this)
    }
}

impl<L, const N: usize> Load for [L; N]
where
    L: AsRepr,
    [L::Repr; N]: Load,
{
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        L::from_repr_array(stream.load()?) //
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl<L1, L2> Load for (L1, L2)
where
    L1: Load,
    L2: Load,
{
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok((stream.load()?, stream.load()?))
    }
}

impl<L1, L2, L3> Load for (L1, L2, L3)
where
    L1: Load,
    L2: Load,
    L3: Load,
{
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok((stream.load()?, stream.load()?, stream.load()?))
    }
}

macro_rules! impl_load {
    ($($T:ty)*) => {
        $(
            impl Load for $T {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.cursor.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const N: usize> Load for [$T; N] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.cursor.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const M: usize, const N: usize> Load for [[$T; M]; N] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.cursor.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const M: usize, const N: usize, const O: usize> Load for [[[$T; M]; N]; O] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.cursor.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
        )*
    }
}
impl_load! { i8 u8 i16 u16 f32 i32 u32 f64 i64 u64 }

pub trait LoadFn: Iterator {
    fn load<L, F, T>(&mut self, function: F) -> io::Result<T>
    where
        T: FromIterator<L>,
        F: FnMut(Self::Item) -> io::Result<L>,
    {
        self.map(function).collect()
    }
}

impl LoadFn for std::ops::Range<u16> {}
impl LoadFn for std::ops::Range<u32> {}
