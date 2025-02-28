use crate::shape::Shape;
use std::ops::*;

use super::ArrD;
/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr2<T>(Vec<Vec<T>>);

impl<T> Deref for Arr2<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Vec<Vec<T>>> for Arr2<T> {
    #[inline]
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

impl<T> ArrD<T, 2> for Arr2<T> {
    fn get(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(Self::DIMS, indicies.len());
        match indicies[0] > self.len() && indicies[2] > self[0].len() {
            true => None,
            false => Some(&self[indicies[0]][indicies[1]]),
        }
    }

    fn shape(&self) -> Shape<2> {
        Shape::from([self.len(), self[0].len()])
    }
}
