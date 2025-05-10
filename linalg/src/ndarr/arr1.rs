use super::tensor::Tensor;
use crate::{
    number::RealFuncs,
    shape::{Shape, ShapeDescriptor},
};
use std::{iter::Sum, ops::*};

///====================== Arr1 ======================
#[derive(Clone)]
pub struct Arr1<'a, T>(Tensor<'a, T>);

impl<'a, T> Arr1<'a, T> {
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

///====================== Arr1 Shape ======================
impl<'a, T> Shape for Arr1<'a, T> {
    #[inline(always)]
    fn shape(&self) -> ShapeDescriptor {
        ShapeDescriptor::from(vec![self.0.len()].into_boxed_slice())
    }

    #[inline(always)]
    fn hypervolume(&self) -> usize {
        self.0.len()
    }

    fn rank(&self) -> usize {
        1
    }
}

///====================== Arr1 Deref ======================
impl<'a, T> Deref for Arr1<'a, T> {
    type Target = Box<[T]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///====================== Arr1 DerefMut ======================
impl<'a, T> DerefMut for Arr1<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///====================== Arr1 From<Vec<U>> ======================
impl<'a, T> From<Vec<T>> for Arr1<'a, T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self::new(value.into())
    }
}
