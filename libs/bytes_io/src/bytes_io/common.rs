#[doc(hidden)]
pub unsafe trait AsRepr
where
    Self: Copy + Sized + Default + TryFrom<Self::Repr>,
    Self::Repr: Copy + Sized,
{
    type Repr;

    fn as_repr_array<const N: usize>(array: &[Self; N]) -> &[Self::Repr; N] {
        unsafe { &*array.as_ptr().cast() }
    }
}
