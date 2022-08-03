// rust std imports
use std::io::{self, Write};

// external imports
use bstr::BString;
use bytemuck::bytes_of;

// internal imports
use crate::bytes_io::{AsRepr, Writer};

pub trait Save: Sized {
    fn save(&self, stream: &mut Writer) -> io::Result<()>;
}

impl Save for String {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save_string(self)
    }
}

impl Save for BString {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save_as::<_, u32>(self.len())?;
        stream.cursor.write_all(self.as_slice())
    }
}

impl<S: Save> Save for Box<S> {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(self.as_ref())
    }
}

impl<S: Save> Save for Vec<S> {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save_as::<_, u32>(self.len())?;
        for item in self {
            stream.save(item)?;
        }
        Ok(())
    }
}

impl<S, const N: usize> Save for [S; N]
where
    S: AsRepr,
    [S::Repr; N]: Save,
{
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(S::as_repr_array(self))?;
        Ok(())
    }
}

impl<S1, S2> Save for (S1, S2)
where
    S1: Save,
    S2: Save,
{
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.0)?;
        stream.save(&self.1)?;
        Ok(())
    }
}

impl<S1, S2, S3> Save for (S1, S2, S3)
where
    S1: Save,
    S2: Save,
    S3: Save,
{
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.0)?;
        stream.save(&self.1)?;
        stream.save(&self.2)?;
        Ok(())
    }
}

macro_rules! impl_save {
    ($($T:ty)*) => {
        $(
            impl Save for $T {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.cursor.write_all(bytes_of(self))
                }
            }
            impl<const N: usize> Save for [$T; N] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.cursor.write_all(bytes_of(self))
                }
            }
            impl<const M: usize, const N: usize> Save for [[$T; M]; N] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.cursor.write_all(bytes_of(self))
                }
            }
            impl<const M: usize, const N: usize, const O: usize> Save for [[[$T; M]; N]; O] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.cursor.write_all(bytes_of(self))
                }
            }
        )*
    }
}
impl_save! { i8 u8 i16 u16 f32 i32 u32 f64 i64 u64 }

#[cfg(feature = "nalgebra")]
const _: () = {
    use bytemuck::Pod;
    use nalgebra::{allocator::Allocator, DefaultAllocator, DimName, OMatrix, Scalar};

    impl<S, R, C> Save for OMatrix<S, R, C>
    where
        Self: Pod,
        S: Scalar,
        R: DimName,
        C: DimName,
        DefaultAllocator: Allocator<S, R, C>,
    {
        fn save(&self, stream: &mut Writer) -> io::Result<()> {
            stream.cursor.write_all(bytes_of(self))?;
            Ok(())
        }
    }
};
