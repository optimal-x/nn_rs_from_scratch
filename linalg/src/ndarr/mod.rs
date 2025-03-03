pub mod arr1;
pub mod arr2;
pub mod nested_arr;

use crate::shape::Shape;

pub trait ArrD<T, const DIMS: usize> {
    fn get(&self, indicies: &[usize]) -> Option<&T>;
    fn shape(&self) -> Shape<DIMS>;
    fn axis(&self) -> usize {
        DIMS
    }
}
