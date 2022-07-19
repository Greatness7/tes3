#[doc(hidden)]
pub unsafe trait AsRepr
where
    Self: Copy + Sized + TryFrom<Self::Repr>,
    Self::Repr: Copy + Sized,
{
    type Repr;

    fn as_repr_array<const N: usize>(array: &[Self; N]) -> &[Self::Repr; N] {
        unsafe { &*array.as_ptr().cast() }
    }

    fn from_repr_array<const N: usize>(array: [Self::Repr; N]) -> Result<[Self; N], &'static str> {
        for value in &array {
            if Self::try_from(*value).is_err() {
                return Err("invalid enum variant");
            }
        }
        Ok(unsafe { *array.as_ptr().cast() })
    }
}
