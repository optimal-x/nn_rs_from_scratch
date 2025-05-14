pub(crate) use super::transform::Transform;
use crate::shape::{Shape, ShapeDescriptor};
use std::{borrow::Cow, marker::PhantomData};

// ======================= Container =======================
/// .
#[derive(Clone)]
pub struct Tensor<'a, T> {
    dtype: PhantomData<T>,
    transform: Option<&'a dyn Transform>,
    data: Box<[T]>,
    shape: ShapeDescriptor,
    strides: Box<[usize]>,
}

impl<'a, T> Tensor<'a, T> {
    pub fn new(data: Box<[T]>, shape: ShapeDescriptor) -> Self {
        let strides = shape.compute_strides();
        Self {
            dtype: PhantomData::<T>,
            transform: None,
            data,
            shape,
            strides,
        }
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    pub fn transform(&self) -> Option<&dyn Transform> {
        if let Some(transform) = self.transform {
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
    pub fn set_transform(&mut self, transform: &'a dyn Transform) {
        self.transform = Some(transform);
    }
}

impl<T> Shape for Tensor<'_, T> {
    fn rank(&self) -> usize {
        self.shape.rank()
    }

    fn shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Borrowed(&self.shape)
    }

    fn hypervolume(&self) -> usize {
        self.shape.hypervolume()
    }
}

impl<T> std::ops::Deref for Tensor<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl<T> std::ops::DerefMut for Tensor<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
