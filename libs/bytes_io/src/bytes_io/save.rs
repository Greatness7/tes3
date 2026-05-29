// rust std imports
use std::io::{self, Write};

// external imports
use bstr::BString;
use half::f16;

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
        stream.save_as::<u32>(self.len())?;
        stream.write_all(self.as_slice())
    }
}

impl<S: Save> Save for Box<S> {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(self.as_ref())
    }
}

impl<S, const N: usize> Save for [S; N]
where
    S: AsRepr,
    [S::Repr; N]: Save,
{
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(S::as_repr_array(self))
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
                    stream.save_pod(self)
                }
            }
            impl<const N: usize> Save for [$T; N] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.save_pod(self)
                }
            }
            impl<const M: usize, const N: usize> Save for [[$T; M]; N] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.save_pod(self)
                }
            }
            impl<const M: usize, const N: usize, const O: usize> Save for [[[$T; M]; N]; O] {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    stream.save_pod(self)
                }
            }
        )*
    }
}
impl_save! { i8 u8 i16 u16 f16 f32 i32 u32 f64 i64 u64 }

#[cfg(feature = "glam")]
const _: () = {
    use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4, Vec4Swizzles};

    macro_rules! impl_save {
        ($($T:ty)*) => {
            $(
                impl Save for $T {
                    fn save(&self, stream: &mut Writer) -> io::Result<()> {
                        stream.save_pod(self)
                    }
                }
            )*
        };
    }
    impl_save! { Vec2 Vec3 Vec4 Mat2 Mat3 Mat4 }

    impl Save for Quat {
        fn save(&self, stream: &mut Writer) -> io::Result<()> {
            stream.save(&Vec4::from(*self).wxyz())
        }
    }
};

/// Specialized `Save` implementation for Vec with fast path for POD scalar types.
const _: () = {
    use castaway::match_type;

    macro_rules! save_vec_fast_path {
        ($stream:expr, $value:expr; $($T:ty)*) => {
            match_type!($value.as_slice(), {
                $(
                    &[$T] as value => $stream.save_vec(value),
                )*
                _ => {
                    for item in $value {
                        $stream.save(item)?;
                    }
                    Ok(())
                },
            })
        };
    }
    impl<S: Save + 'static> Save for Vec<S> {
        fn save(&self, stream: &mut Writer) -> io::Result<()> {
            stream.save_as::<u32>(self.len())?;
            save_vec_fast_path! { stream, self; i8 u8 i16 u16 f16 f32 i32 u32 f64 i64 u64 }
        }
    }
};
