use super::{container::Container, transform::Transform};
use std::marker::PhantomData;

pub struct Tensor<'a, T, Ct>
where
    Ct: Container<T>,
{
    dtype: PhantomData<T>,
    transform: Option<&'a dyn Transform>,
    data: Ct,
}

impl<'a, T, Ct> Tensor<'a, T, Ct>
where
    Ct: Container<T>,
{
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
