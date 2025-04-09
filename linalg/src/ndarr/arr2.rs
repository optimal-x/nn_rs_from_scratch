use crate::{
    number::{Comp, RealFuncs},
    shape::Shape,
};
use std::ops::*;

use super::ArrD;
/// Owner of 1D-array data.
#[derive(Debug, Clone)]
pub struct Arr2<T>(Vec<Vec<Comp<T>>>);

impl<T> Arr2<T> {
    #[inline]
    pub fn rows(&self) -> usize {
        self.len()
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.first().map_or(0, |row| row.len())
    }

    pub fn matmul(self, rhs: Self) -> Self
    where
        T: RealFuncs,
    {
        let (m, n) = (self.rows(), self.cols());
        let (n_rhs, p) = (rhs.rows(), rhs.cols());

        // checking for at least 1 matching dim.
        assert_eq!(
            n, n_rhs,
            "[[linalg]] Matrix multiplication dimensions mismatch"
        );

        let mut result = vec![vec![T::default(); p]; m];
        for i in 0..m {
            for j in 0..p {
                result[i][j] = (0..n).map(|k| self[i][k] * rhs[k][j]).sum();
            }
        }

        Arr2::from(result)
    }

    pub fn matadd(self, rhs: Self) -> Self
    where
        T: RealFuncs,
    {
        let shape = self.shape();
        assert_eq!(
            shape,
            rhs.shape(),
            "[[linalg]] Matrix Addition dimensions mismatch"
        );

        let (m, n) = (shape[0], shape[1]);

        let mut collector: Vec<Vec<T>> = vec![Vec::with_capacity(n); m];
        for i in 0..m {
            for j in 0..n {
                collector[i][j] = self[i][j] + rhs[i][j];
            }
        }
        Arr2::from(collector)
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
        assert_eq!(self.rank(), indicies.len());
        match indicies[0] > self.len() && indicies[2] > self[0].len() {
            true => None,
            false => Some(&self[indicies[0]][indicies[1]]),
        }
    }

    fn shape(&self) -> Shape<2> {
        Shape::from([self.len(), self[0].len()])
    }
}

impl<T> Mul for Arr2<T> {
    type Output = Arr2<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.matmul(rhs)
    }
}

impl<T> Add for Arr2<T> {
    type Output = Arr2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.matadd(rhs)
    }
}
