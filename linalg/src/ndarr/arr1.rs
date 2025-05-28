use crate::ndarr::tensor::TensorAccess;
use super::{tensor::Tensor, transform::compute_flat_index};
use crate::{
    number::RealFuncs,
    shape::{Shape, ShapeDescriptor},
};
use std::{borrow::Cow, iter::Sum, ops::*};

//====================== Arr1 ======================
#[derive(Clone)]
pub struct Arr1<'a, T>(Tensor<'a, T>);

impl<T: Clone> Arr1<'_, T> {
    pub fn new(data: Box<[T]>) -> Self {
        let size = data.len();
        Self(Tensor::new(data, ShapeDescriptor(Box::new([size]))))
    }

    pub fn distance(&self, rhs: &Self) -> T
    where
        T: RealFuncs<T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Sum<T>,
        T: Clone + Copy,
    {
        assert_eq!(self.0.len(), rhs.0.len());
        let s: T = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&r, &l)| (r - l) * (r - l))
            .sum();
        s.sqrt()
    }

    pub fn manhattan(&self, rhs: &Self) -> T
    where
        T: Mul<Output = T>,
        T: Default,
        T: Sub<Output = T>,
        T: Neg<Output = T>,
        T: PartialOrd,
        T: Sum<T>,
        T: Clone + Copy,
    {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(r, l)| {
                let diff = *r - *l;
                if diff < T::default() { -diff } else { diff }
            })
            .sum()
    }

    pub fn inner_product(&self, rhs: &Self) -> T
    where
        T: Mul<Output = T>,
        T: Sum<T>,
        T: Clone + Copy,
    {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0.iter().zip(rhs.0.iter()).map(|(r, l)| *r * *l).sum()
    }
}

//====================== Arr1 Shape ======================
impl<T> Shape for Arr1<'_, T> {
    #[inline(always)]
    fn shape(&self) -> Cow<ShapeDescriptor> {
        self.0.shape()
    }

    #[inline(always)]
    fn hypervolume(&self) -> usize {
        self.0.len()
    }

    fn rank(&self) -> usize {
        1
    }
}

//====================== Arr1 Deref ======================
impl<'a, T> Deref for Arr1<'a, T> {
    type Target = Tensor<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//====================== Arr1 DerefMut ======================
impl<T> DerefMut for Arr1<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//====================== Arr1 From<Vec<T>> ======================
impl<T: Clone> From<Vec<T>> for Arr1<'_, T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self::new(value.into())
    }
}

//====================== Arr1 From<Box<[T]> ======================
impl<T: Clone> From<Box<[T]>> for Arr1<'_, T> {
    #[inline]
    fn from(value: Box<[T]>) -> Self {
        Self::new(value)
    }
}

//====================== Arr1 Index ======================
impl<T> Index<[usize; 1]> for Arr1<'_, T> {
    type Output = T;

    fn index(&self, logical: [usize; 1]) -> &Self::Output {
        let strides = self.strides();
        let flat = compute_flat_index(&logical, strides);
        &self.0.data()[flat]
    }
}

//====================== Arr1 IndexMut ======================
impl<T> IndexMut<[usize; 1]> for Arr1<'_, T> {
    fn index_mut(&mut self, logical: [usize; 1]) -> &mut Self::Output {
        let strides = self.strides();
        let flat = compute_flat_index(&logical, strides);
        &mut self.0.data_mut()[flat]
    }
}
