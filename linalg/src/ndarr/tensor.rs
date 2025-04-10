use crate::shape::{Shape, StructureShape};
use std::marker::PhantomData;

pub trait Container<T, const DIM: usize>: Shape<DIM> {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&self, indicies: &[usize], value: T);
}

pub struct Tensor<T, Ct, const DIM: usize>
where
    Ct: Container<T, DIM>,
{
    dtype: PhantomData<T>,
    shape: StructureShape<DIM>,
    data: Ct,
}

impl<T, Ct, const DIM: usize> Tensor<T, Ct, DIM>
where
    Ct: Container<T, DIM>,
{
    pub const RANK: usize = DIM;

    pub fn new(data: Ct) -> Self {
        Self {
            dtype: PhantomData::<T>,
            shape: data.shape(),
            data,
        }
    }

    pub fn shape(&self) -> &StructureShape<DIM> {
        &self.shape
    }

    pub fn data(&self) -> &Ct {
        &self.data
    }
}
