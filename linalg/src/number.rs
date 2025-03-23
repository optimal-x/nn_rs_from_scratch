#![allow(dead_code)]
// use nnrs_macros::cast_number;
use std::ops::*;

pub trait GetSetDelegate {
    fn get(&self) -> Box<dyn GetSetDelegate>;
    fn set(&self, value: Box<dyn GetSetDelegate>);
}

pub trait NaturalFuncs<T> {
    fn abs(self) -> Comp<T>;
    fn signum(self) -> Comp<T>;
}

pub trait RealFuncs<T>: NaturalFuncs<T> {
    fn sqrt(self) -> Comp<T>;
    fn cbrt(self) -> Comp<T>;
    fn powi(self, n: i32) -> Comp<T>;
    fn powf(self, n: Comp<T>) -> Comp<T>;
    fn exp(self) -> Comp<T>;
    fn exp2(self) -> Comp<T>; //2^x
    fn ln(self) -> Comp<T>;
    fn log(self, base: Comp<T>) -> Comp<T>;
    fn log2(self) -> Comp<T>;
    fn log10(self) -> Comp<T>;
    fn floor(self) -> Comp<T>;
    fn ceil(self) -> Comp<T>;
    fn round(self) -> Comp<T>;
    fn trunc(self) -> Comp<T>;
    fn fract(self) -> Comp<T>;
    fn mul_add(self, a: Comp<T>, b: Comp<T>) -> Comp<T>;
    fn max(self, other: Comp<T>) -> Comp<T>;
    fn min(self, other: Comp<T>) -> Comp<T>;
}

pub struct Comp<T>(T);
impl<T> Comp<T> {
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.0
    }

    #[inline(always)]
    pub fn set(&mut self, value: T) {
        self.0 = value;
    }
}

impl<T> Deref for Comp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! impl_integerfuncs_for_comp_t {
    ($ty:ty) => {
        impl NaturalFuncs<$ty> for Comp<$ty> {
            fn abs(self) -> Comp<$ty> {
                Self(self.get().abs())
            }

            fn signum(self) -> Comp<$ty> {
                Self(self.get().signum())
            }
        }
    };
    ($($ty:ty),+) => {
        $(impl_integerfuncs_for_comp_t!($ty);)*
    };
}
macro_rules! impl_numberfuncs_for_comp_t {
    ($ty:ty) => {
        impl RealFuncs<$ty> for Comp<$ty> {
            fn sqrt(self) -> Comp<$ty> { Self(self.get().sqrt()) }
            fn cbrt(self) -> Comp<$ty> { Self(self.get().cbrt()) }
            fn powi(self, n: i32) -> Comp<$ty> { Self(self.get().powi(n)) }
            fn powf(self, n: Comp<$ty>) -> Comp<$ty> { Self(self.get().powf(*n.get())) }
            fn exp(self) -> Comp<$ty> { Self(self.get().exp()) }
            fn exp2(self) -> Comp<$ty> { Self(self.get().exp2()) }
            fn ln(self) -> Comp<$ty> { Self(self.get().ln()) }
            fn log(self, base: Comp<$ty>) -> Comp<$ty> { Self(self.get().log(*base.get())) }
            fn log2(self) -> Comp<$ty> { Self(self.get().log2()) }
            fn log10(self) -> Comp<$ty> { Self(self.get().log10()) }
            fn floor(self) -> Comp<$ty> { Self(self.get().floor()) }
            fn ceil(self) -> Comp<$ty> { Self(self.get().ceil()) }
            fn round(self) -> Comp<$ty> { Self(self.get().round()) }
            fn trunc(self) -> Comp<$ty> { Self(self.get().trunc()) }
            fn fract(self) -> Comp<$ty> { Self(self.get().fract()) }
            fn mul_add(self, a: Comp<$ty>, b: Comp<$ty>) -> Comp<$ty> { Self(self.get().mul_add(*a.get(), *b.get())) }
            fn max(self, other: Comp<$ty>) -> Comp<$ty> { Self(self.get().max(*other.get())) }
            fn min(self, other: Comp<$ty>) -> Comp<$ty> { Self(self.get().min(*other.get())) }
        }
    };
    ($($ty:ty),+) => {
        $(impl_numberfuncs_for_comp_t!($ty);)*
    };
}

macro_rules! in_operator {
    ($($t:ty),+) => {
        $(
            impl Add for $t {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }
            impl Mul for $t {
                type Output = Self;
                fn mul(self, rhs: Self) -> Self::Output {
                    Self(self.0 * rhs.0)
                }
            }
            impl Div for $t {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self(self.0 / rhs.0)
                }
            }
            impl Neg for $t {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self(-self.0)
                }
            }
            impl Sub for $t {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self(self.0 - rhs.0)
                }
            }
            impl Sum for $t {
                fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                    Self(iter.map(|i| i.0).sum())
                }
            }
        )*
    };
}

// impls basic operators for each of the types (eg. +, -, /, *)
// impl_number![F64, F32, I64, I32, I16, I8];
// cast_number![F64, F32, I64, I32, I16, I8];
// in_operator![F64, F32, I64, I32, I16, I8];
impl_integerfuncs_for_comp_t![f32, f64, i32, i64, i128];
impl_numberfuncs_for_comp_t![f32, f64];
