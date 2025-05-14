
use crate::{
    ndarr::transform::{
        compute_flat_index, default_slice,
    },
    number::RealFuncs,
    shape::{Shape, ShapeDescriptor},
};
use std::{borrow::Cow, iter::Sum, ops::*};

use super::tensor::Tensor;

//====================== Arr2 ======================
pub struct Arr2<'a, T>(Tensor<'a, T>);


impl<T> Arr2<'_, T> {
    pub fn new(data: Box<[T]>, shape: (usize, usize)) -> Self {
        Self(Tensor::new(data, ShapeDescriptor(Box::new([shape.0, shape.1]))))
    }
    
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

        let mut buff: Box<[T]> = default_slice(m * p);
        let strides = self.strides();

        for i in 0..m {
            for j in 0..p {
                let logical = [i, j];
                let flat = compute_flat_index(&logical, strides);

                (0..n).for_each(|k| {
                    buff[flat] = self[[i, k]] * rhs[[k, j]];
                });
            }
        }

        Arr2::new(buff, (m, p))
    }

    pub fn matadd(self, rhs: Self) -> Self
    where
        T: Clone + Copy,
        T: Add<Output = T>,
        T: Default,
    {
        let shape = self.shape();
        assert_eq!(
            shape,
            rhs.shape(),
            "[[linalg]] Matrix Addition dimensions mismatch"
        );

        let (m, n) = (shape[0], shape[1]);

        let mut buff: Box<[T]> = default_slice(m * n);
        let strides = self.strides();
        for i in 0..m {
            for j in 0..n {
                let logical = [i, j];
                let flat = compute_flat_index(&logical, strides);
                buff[flat] = self[[i, j]] + rhs[[i, j]];
            }
        }
        
        Arr2::new(buff, (m,n))
    }
}

///====================== Arr2 Shape ======================
impl<T> Shape for Arr2<'_, T> {
    fn shape(&self) -> Cow<ShapeDescriptor >{
        self.0.shape()
    }

    fn hypervolume(&self) -> usize {
        self.shape()[0] * self.shape()[1]
    }

    fn rank(&self) -> usize {
        2
    }
}

///====================== Arr2 Deref ======================
impl<'a, T> Deref for Arr2<'a, T> {
    type Target = Tensor<'a, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
 
///====================== Arr2 DerefMut ======================
impl<T> DerefMut for Arr2<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///====================== Arr2 From<Vec<Vec<T>>>======================
impl<T> From<Vec<Vec<T>>> for Arr2<'_, T> {
    #[inline]
    fn from(value: Vec<Vec<T>>) -> Self {
        todo!()
    }
}

///====================== Arr2 Mul ======================
impl<'a, T> Mul for Arr2<'a, T>
where
    T: Mul<Output = T>,
    T: Sum<T>,
    T: Default,
    T: RealFuncs<T>,
    T: Clone + Copy,
{
    type Output = Arr2<'a, T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.matmul(rhs)
    }
}

///====================== Arr2 Add ======================
impl<'a, T> Add for Arr2<'a, T>
where
    T: Add<Output = T>,
    T: RealFuncs<T>,
    T: Clone + Copy,
    T: Default
{
    type Output = Arr2<'a, T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.matadd(rhs)
    }
}

///====================== Arr2 Index ======================
impl<T> Index<[usize; 2]> for Arr2<'_, T> {
    type Output = T;

    fn index(&self, logical: [usize; 2]) -> &Self::Output {
        let strides = self.strides();
        let flat = compute_flat_index(&logical, strides);
        &self.0[flat]
    }
}

///====================== Arr2 IndexMut ======================
impl<T> IndexMut<[usize; 2]> for Arr2<'_, T> {
    fn index_mut(&mut self, logical: [usize; 2]) -> &mut Self::Output {
        let strides = self.strides();
        let flat = compute_flat_index(&logical, strides);
        &mut self.0[flat]
    }
}
