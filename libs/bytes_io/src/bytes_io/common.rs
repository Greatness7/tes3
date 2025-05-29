use bytemuck::{NoUninit, Pod};

#[doc(hidden)]
pub trait AsRepr
where
    Self: Default + NoUninit + TryFrom<Self::Repr>,
    Self::Repr: Pod,
{
    type Repr;

    fn as_repr_array<const N: usize>(array: &[Self; N]) -> &[Self::Repr; N] {
        bytemuck::must_cast_slice(array).try_into().unwrap()
    }
}
