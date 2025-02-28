use crate::{number::Number, shape::Shape};
use std::ops::*;

use super::ArrD;
/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr2<T>(Vec<Vec<T>>);

impl<T> Arr2<T> {
    #[inline]
    pub fn rows(&self) -> usize {
        self.len()
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.first().map_or(0, |row| row.len())
    }
}

impl<T> Deref for Arr2<T> {
    type Target = Vec<Vec<T>>;

    #[inline]
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

impl<T> Mul for Arr2<T>
where
    T: Number,
{
    type Output = Arr2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (m, n) = (self.rows(), self.cols());
        let (n_rhs, p) = (rhs.rows(), rhs.cols());

        assert_eq!(
            n, n_rhs,
            "[[linalg]] Matrix multiplication dimensions mismatch"
        );

        let mut result = vec![vec![T::default(); p]; m];

        (0..m).for_each(|i| {
            (0..p).for_each(|j| {
                result[i][j] = (0..n).map(|k| self[i][k] * rhs[k][j]).sum();
            })
        });

        Arr2::from(result)
    }
}
