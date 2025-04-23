use super::{container::Container, transform::Transform};
use std::marker::PhantomData;

pub struct Tensor<'a, T, Ct, const DIM: usize>
where
    Ct: Container<T, DIM>,
{
    dtype: PhantomData<T>,
    transform: Option<&'a dyn Transform>,
    data: Ct,
}

impl<'a, T, Ct, const DIM: usize> Tensor<'a, T, Ct, DIM>
where
    Ct: Container<T, DIM>,
{
    pub const RANK: usize = DIM;

    pub fn new(data: Ct) -> Self {
        Self {
            dtype: PhantomData::<T>,
            transform: None,
            data,
        }
    }

    pub fn data(&self) -> &Ct {
        &self.data
    }

    pub fn transform(&self) -> Option<&dyn Transform> {
        if let Some(transform) = self.transform.as_deref() {
            Some(transform)
        } else {
            None
        }
    }

    pub fn set_transform(&mut self, transform: &'a dyn Transform) {
        self.transform = Some(transform);
    }
}
