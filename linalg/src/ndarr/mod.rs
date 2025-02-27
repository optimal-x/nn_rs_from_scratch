pub mod arr1;

use crate::number::Number;

pub trait ArrD<T: Number, const DIMS: usize> {
    const DIMS: usize = DIMS;
    fn get(&self, x: &[usize]) -> Option<&T>;
    fn dot(&self, rhs: &Self) -> T;
    fn manhattan(&self, rhs: &Self) -> T; // Manhattan distance
    fn distance(&self, rhs: &Self) -> T; // Euclidean distance
}

// Handle to 1D-array data.
// #[derive(Debug, Clone)]
// pub struct Arr1Handle<T: Number>(Rc<Vec<T>>);

// impl<T> From<&Vec<T>> for Arr1Handle<T>
// where
//     T: Number,
// {
//     #[inline]
//     fn from(value: &Vec<T>) -> Self {
//         Self(Rc::new(value.to_vec()))
//     }
// }
