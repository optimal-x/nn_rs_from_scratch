use super::ArrD;
use crate::number::{Number, NumberFuncs};
use crate::shape::Shape;
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

impl<T> DerefMut for Arr1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for Arr1<T>
where
    T: Number,
{
    #[inline]
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T> Arr1<T> {
    pub fn new(container: Vec<T>) -> Self {
        Self(container)
    }

    pub fn distance(&self, rhs: &Self) -> T
    where
        T: Number,
        T: NumberFuncs,
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
        T: Number,
    {
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

    pub fn inner_product(&self, rhs: &Self) -> T
    where
        T: Number,
    {
        assert_eq!(self.0.len(), rhs.0.len());
        self.0.iter().zip(rhs.0.iter()).map(|(r, l)| *r * *l).sum()
    }
}

impl<T> ArrD<T, 1> for Arr1<T> {
    fn get(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(self.axis(), indicies.len());
        match indicies[0] > self.len() {
            true => None,
            false => Some(&self[indicies[0]]),
        }
    }

    fn shape(&self) -> Shape<1> {
        Shape::from([self.len()])
    }
}

/// Most types are trivially copiable but this is a recurive trait meaning
/// that any T including ,but not-limited to Arr1, can call any NumberFuncs methods
/// this means that technically for higher dimensional structures T might not be
/// trivial to copy.
impl<T> NumberFuncs for Arr1<T>
where
    T: NumberFuncs + Clone,
{
    fn sqrt(mut self) -> Self {
        for i in 0..self.axis() {
            self[i] = self[i].clone().sqrt();
        }
        self
    }

    fn cbrt(mut self) -> Self {
        for i in 0..self.axis() {
            self[i] = self[i].clone().cbrt();
        }
        self
    }

    fn powi(mut self, n: i32) -> Self {
        for i in 0..self.axis() {
            self[i] = self[i].clone().powi(n);
        }
        self
    }

    fn powf<Num: Number>(mut self, n: Num) -> Self {
        for i in 0..self.axis() {
            self[i] = self[i].clone().powf(n);
        }
        self
    }

    fn exp(self) -> Self {
        todo!()
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        todo!()
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn abs(self) -> Self {
        todo!()
    }

    fn signum(self) -> Self {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    fn max(self, other: Self) -> Self {
        todo!()
    }

    fn min(self, other: Self) -> Self {
        todo!()
    }
}
