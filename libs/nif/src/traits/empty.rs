use nalgebra::{DimName, Dyn, OMatrix, Scalar};

pub trait Empty {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
}

impl<T> Empty for Vec<T> {
    fn empty() -> Self {
        vec![]
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: Scalar, U: DimName> Empty for OMatrix<T, U, Dyn> {
    fn empty() -> Self {
        Self::from_vec(vec![])
    }
    fn is_empty(&self) -> bool {
        self.ncols() == 0
    }
}
