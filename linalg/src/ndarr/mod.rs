pub mod arr1;
pub mod arr2;
pub mod nested_arr;
pub mod tensor;

use crate::shape::Shape;

pub trait ArrD<T, const DIM: usize>: Shape<DIM> {
    fn get(&self, indicies: &[usize]) -> Option<&T>;
}

