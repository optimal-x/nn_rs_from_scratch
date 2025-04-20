use super::container::Container;
use crate::{
    number::RealFuncs,
    shape::{Shape, StructureShape},
};
use std::{iter::Sum, ops::*};

///====================== Arr1 ======================
#[derive(Debug, Clone)]
pub struct Arr1<T>(Vec<T>);

impl<T> Arr1<T> {
    pub fn new(container: Vec<T>) -> Self {
        Self(container)
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
impl<T> Shape<1> for Arr1<T> {
    #[inline(always)]
    fn shape(&self) -> StructureShape<1> {
        StructureShape::<1>::from([self.0.len()])
    }

    #[inline(always)]
    fn n_volume(&self) -> usize {
        self.0.len()
    }
}

///====================== Arr1 Container ======================
impl<T> Container<T, 1> for Arr1<T> {
    fn at(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(indicies.len(), Self::RANK);
        todo!() // TODO
    }

    fn set_at(&self, indicies: &[usize], value: T) {
        assert_eq!(indicies.len(), Self::RANK);
        todo!() // TODO
    }
}

///====================== Arr1 Deref ======================
impl<T> Deref for Arr1<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///====================== Arr1 DerefMut ======================
impl<T> DerefMut for Arr1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///====================== Arr1 From<Vec<U>> ======================
impl<T> From<Vec<T>> for Arr1<T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}
