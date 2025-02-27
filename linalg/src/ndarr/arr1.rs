use crate::number::{Number, NumberFuncs};
use std::iter::Sum;
use std::ops::*;

use super::ArrD;
/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr1<T: Number>(Vec<T>);

impl<T> Deref for Arr1<T>
where
    T: Number,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Vec<T>> for Arr1<T>
where
    T: Number,
{
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> ArrD<T, 1> for Arr1<T>
where
    T: Number
        + NumberFuncs
        + Sum
        + Copy
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>,
{
    fn distance(&self, rhs: &Self) -> T {
        assert_eq!(self.0.len(), rhs.0.len());
        let s: T = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&r, &l)| (r - l) * (r - l))
            .sum();
        s.sqrt()
    }

    fn dot(&self, rhs: &Self) -> T {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0.iter().zip(rhs.0.iter()).map(|(r, l)| *r * *l).sum()
    }

    fn manhattan(&self, rhs: &Self) -> T {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(r, l)| {
                let diff = *r - *l;
                if diff < T::default() {
                    -diff
                } else {
                    diff
                }
            })
            .sum()
    }

    fn get(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(Self::DIMS, indicies.len());
        match indicies[0] > self.len() {
            true => None,
            false => Some(&(self as &Vec<_>)[indicies[0]]),
        }
    }
}
