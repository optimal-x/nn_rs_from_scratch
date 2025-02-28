use super::ArrD;
use crate::number::{Number, NumberFuncs};
use crate::shape::Shape;
use std::iter::Sum;
use std::ops::*;

/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr1<T>(Vec<T>);

impl<T> Deref for Arr1<T> {
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

impl<T> Arr1<T>
where
    T: Number,
    T: NumberFuncs,
    T: Sum,
    T: Copy,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
{
    pub fn distance(&self, rhs: &Self) -> T {
        assert_eq!(self.0.len(), rhs.0.len());
        let s: T = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&r, &l)| (r - l) * (r - l))
            .sum();
        s.sqrt()
    }

    pub fn manhattan(&self, rhs: &Self) -> T {
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

    pub fn inner_product(&self, rhs: &Self) -> T {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0.iter().zip(rhs.0.iter()).map(|(r, l)| *r * *l).sum()
    }
}

impl<T> ArrD<T, 1> for Arr1<T> {
    fn get(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(Self::DIMS, indicies.len());
        match indicies[0] > self.len() {
            true => None,
            false => Some(&self[indicies[0]]),
        }
    }

    fn shape(&self) -> Shape<1> {
        Shape::from([self.len()])
    }
}
