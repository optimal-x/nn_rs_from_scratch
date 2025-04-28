use crate::{
    number::RealFuncs,
    shape::{Shape, ShapeDescriptor},
};
use std::{iter::Sum, ops::*};

///====================== Arr2 ======================
#[derive(Debug, Clone)]
pub struct Arr2<T>(Vec<Vec<T>>);

impl<T> Arr2<T> {
    #[inline]
    pub fn rows(&self) -> usize {
        self.shape()[0]
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.shape()[1]
    }

    pub fn matmul(self, rhs: Self) -> Self
    where
        T: Clone + Copy,
        T: Mul<Output = T>,
        T: Default,
        T: Sum<T>,
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
        T: Clone + Copy,
        T: Add<Output = T>,
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

///====================== Arr2 Shape ======================
impl<T> Shape for Arr2<T> {
    fn shape(&self) -> ShapeDescriptor {
        ShapeDescriptor::from(vec![self.len(), self[0].len()].into_boxed_slice())
    }

    fn hypervolume(&self) -> usize {
        self.shape()[0] * self.shape()[1]
    }

    fn rank(&self) -> usize {
        2
    }
}

///====================== Arr2 Deref ======================
impl<T> Deref for Arr2<T> {
    type Target = Vec<Vec<T>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///====================== Arr2 Deref ======================
impl<T> From<Vec<Vec<T>>> for Arr2<T> {
    #[inline]
    fn from(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
}

///====================== Arr2 Mul ======================
impl<T> Mul for Arr2<T>
where
    T: Mul<Output = T>,
    T: Sum<T>,
    T: Default,
    T: RealFuncs<T>,
    T: Clone + Copy,
{
    type Output = Arr2<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.matmul(rhs)
    }
}

///====================== Arr2 Add ======================
impl<T> Add for Arr2<T>
where
    T: Add<Output = T>,
    T: RealFuncs<T>,
    T: Clone + Copy,
{
    type Output = Arr2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.matadd(rhs)
    }
}
