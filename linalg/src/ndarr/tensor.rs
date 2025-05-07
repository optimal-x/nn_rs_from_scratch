use super::{container::Device, transform::Transform};
use std::marker::PhantomData;

// ======================= Container =======================
/// .
pub struct Tensor<'a, T, Ct>
where
    Ct: Device<T>,
{
    dtype: PhantomData<T>,
    transform: Option<&'a dyn Transform>,
    data: Ct,
}

impl<'a, T, Ct> Tensor<'a, T, Ct>
where
    Ct: Device<T>,
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

    /// The intention behind the transform is for the it and the tensor
    /// to be losely connected s.t. the transform can technically exist as its
    /// own object without guidance of any one tensor.
    ///
    /// This means the a transform can technically be used on another tensor
    /// without needing a tensor to start with. Also meaning that you can
    /// do arbirary shaping and manipulation without a Tensor.
    #[must_use]
    pub fn set_transform(&mut self, transform: &'a dyn Transform) {
        self.transform = Some(transform);
    }
}

impl<'a, T, Ct> std::ops::Index<&[usize]> for Tensor<'a, T, Ct>
where
    Ct: Device<T>,
{
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        todo!()
    }
}

impl<'a, T, Ct> std::ops::IndexMut<&[usize]> for Tensor<'a, T, Ct>
where
    Ct: Device<T>,
{
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        todo!()
    }
}
