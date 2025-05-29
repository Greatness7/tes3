// rust std imports
use std::io::{self, Read};

// external imports
use bstr::BString;
use bytemuck::{bytes_of_mut, Zeroable};

// internal imports
use crate::bytes_io::{AsRepr, Reader};

pub trait Load: Sized {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self>;
}

impl Load for String {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len = stream.load_as::<u32, usize>()?;
        stream.load_string(len)
    }
}

impl Load for BString {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len = stream.load_as::<u32, usize>()?;
        Ok(stream.load_bytes(len)?.into())
    }
}

impl<L: Load> Load for Box<L> {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        Ok(Box::new(stream.load()?))
    }
}

#[cfg(not(feature = "nightly"))]
impl<L: Load> Load for Vec<L> {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len: u32 = stream.load()?;
        (0..len).map(|_| stream.load()).collect()
    }
}

#[cfg(feature = "nightly")]
impl<L: Load> Load for Vec<L> {
    default fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let len: u32 = stream.load()?;
        (0..len).map(|_| stream.load()).collect()
    }
}

impl<L, const N: usize> Load for [L; N]
where
    L: AsRepr,
    [L::Repr; N]: Load,
{
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let array: [L::Repr; N] = stream.load()?;
        Ok(array.map(|value| value.try_into().unwrap_or_default()))
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
                    stream.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const N: usize> Load for [$T; N] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const M: usize, const N: usize> Load for [[$T; M]; N] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            impl<const M: usize, const N: usize, const O: usize> Load for [[[$T; M]; N]; O] {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let mut this = Self::zeroed();
                    stream.read_exact(bytes_of_mut(&mut this))?;
                    Ok(this)
                }
            }
            #[cfg(feature = "nightly")]
            impl Load for Vec<$T> {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    use bytemuck::{must_cast_slice_mut, zeroed_vec};
                    let len = stream.load_as::<u32, usize>()?;
                    let mut this = zeroed_vec(len);
                    stream.read_exact(must_cast_slice_mut(&mut this))?;
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

#[cfg(feature = "glam")]
const _: () = {
    use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4, Vec4Swizzles};

    macro_rules! impl_load {
        ($($T:ty)*) => {
            $(
                impl Load for $T {
                    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                        let mut this = Self::zeroed();
                        stream.read_exact(bytes_of_mut(&mut this))?;
                        Ok(this)
                    }
                }
            )*
        };
    }
    impl_load! { Vec2 Vec3 Vec4 Mat2 Mat3 Mat4 }

    impl Load for Quat {
        fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
            stream.load().map(Vec4::yzwx).map(Quat::from_vec4)
        }
    }
};
