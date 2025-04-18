use crate::shape::{Shape, StructureShape};
use std::marker::PhantomData;

use super::transform::{compute_strides, Transform};

pub trait Container<T, const DIM: usize>: Shape<DIM> {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&self, indicies: &[usize], value: T);
}

pub struct Tensor<T, Ct, const DIM: usize>
where
    Ct: Container<T, DIM>,
{
    dtype: PhantomData<T>,
    // offset: usize, // maybe needed. Assume offset is 0 for now
    shape: StructureShape<DIM>,
    subscribers: Vec<Transform>,
    strides: Vec<usize>,
    data: Ct,
}

impl<T, Ct, const DIM: usize> Tensor<T, Ct, DIM>
where
    Ct: Container<T, DIM>,
{
    pub const RANK: usize = DIM;

    pub fn new(data: Ct) -> Self {
        let strides = compute_strides(&data.shape());
        Self {
            dtype: PhantomData::<T>,
            shape: data.shape(),
            subscribers: vec![],
            strides,
            data,
        }
    }

    pub fn shape(&self) -> &StructureShape<DIM> {
        &self.shape
    }

    pub fn data(&self) -> &Ct {
        &self.data
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }
}
