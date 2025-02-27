use crate::number::{Number, NumberFuncs};
use std::iter::Sum;
use std::ops::*;

use super::ArrD;
/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr2<T: Number>(Vec<Vec<T>>);

impl<T> Deref for Arr2<T>
where
    T: Number,
{
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Vec<Vec<T>>> for Arr2<T>
where
    T: Number,
{
    #[inline]
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

impl<T> ArrD<T, 2> for Arr2<T>
where
    T: Number
        + NumberFuncs
        + Sum
        + Copy
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>,
{
    fn get(&self, x: &[usize]) -> Option<&T> {
        todo!()
    }

    fn dot(&self, rhs: &Self) -> T {
        todo!()
    }

    fn manhattan(&self, rhs: &Self) -> T {
        todo!()
    }

    fn distance(&self, rhs: &Self) -> T {
        todo!()
    }
}
