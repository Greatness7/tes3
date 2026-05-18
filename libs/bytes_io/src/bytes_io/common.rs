use std::mem::{align_of, size_of};

use bytemuck::{NoUninit, Pod};

#[doc(hidden)]
pub trait AsRepr
where
    Self: Default + NoUninit + TryFrom<Self::Repr>,
    Self::Repr: Pod,
{
    type Repr;

    fn as_repr_array<const N: usize>(array: &[Self; N]) -> &[Self::Repr; N] {
        const {
            assert!(size_of::<Self>() == size_of::<Self::Repr>());
            assert!(align_of::<Self>() == align_of::<Self::Repr>());
        }
        bytemuck::must_cast_slice(array).try_into().unwrap()
    }
}
