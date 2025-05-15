use super::transform::compute_flat_index;
pub(crate) use super::transform::Transform;
use crate::shape::{Shape, ShapeDescriptor};
use std::{borrow::Cow, marker::PhantomData, ops::{Index, IndexMut}};

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
}

// ======================= trait TensorAccess =======================
pub trait TensorAccess<'a, T> {
    fn data(&self) -> &[T];
    fn strides(&self) -> &[usize];

    fn transform(&self) -> Option<&dyn Transform>;

    /// The intention behind the transform is for the it and the tensor
    /// to be losely connected s.t. the transform can technically exist as its
    /// own object without guidance of any one tensor.
    ///
    /// This means the a transform can technically be used on another tensor
    /// without needing a tensor to start with. Also meaning that you can
    /// do arbirary shaping and manipulation without a Tensor.
    fn set_transform(&mut self, transform: &'a dyn Transform); 
}

impl<'a, T> TensorAccess<'a, T> for Tensor<'a, T> {
    fn data(&self) -> &[T] {
        &self.data
    }

    fn strides(&self) -> &[usize] {
        &self.strides
    }

    fn transform(&self) -> Option<&dyn Transform> {
        self.transform
    }
 
    fn set_transform(&mut self, transform: &'a dyn Transform) {
        self.transform = Some(transform);
    }
}

// ======================= impl Shape =======================
impl<T> Shape for Tensor<'_, T> {
    fn rank(&self) -> usize {
        self.shape.rank()
    }

    fn shape(&self) -> Cow<ShapeDescriptor> {
        if let Some(transform) = self.transform {
            transform.out_shape()
        } else {
            Cow::Borrowed(&self.shape)
        }
    }

    fn hypervolume(&self) -> usize {
        self.shape.hypervolume()
    }
}

// ======================= impl Deref =======================
impl<T> std::ops::Deref for Tensor<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

// ======================= impl DerefMut =======================
impl<T> std::ops::DerefMut for Tensor<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}



// ======================= impl Index =======================
impl<'a, T> Index<&[usize]> for Tensor<'a, T> {
    type Output = T;

    fn index(&self, logical: &[usize]) -> &Self::Output {
        let flat = match self.transform {
            Some(t) => t.to_flat(logical),
            None => compute_flat_index(logical, self.strides()),
        };
        &self.data[flat]
    }
}

// ======================= impl IndexMut =======================
impl<'a, T> IndexMut<&[usize]> for Tensor<'a, T> {
    fn index_mut(&mut self, logical: &[usize]) -> &mut Self::Output {
        // TODO remove dups
        let flat = match self.transform {
            Some(t) => t.to_flat(logical),
            None => compute_flat_index(logical, self.strides()),
        };
        &mut self.data[flat]
    }
}

