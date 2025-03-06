#![allow(dead_code)]
use nnrs_macros::cast_number;
use std::{iter::Sum, ops::*};

pub trait NumberFuncs {
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf<Num: Number>(self, n: Num) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self; //2^x
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}

pub trait Number:
    Sized
    + Copy
    + Default
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Sum<Self>
{
    fn cast_to<U>(self) -> U
    where
        U: Number,
        U: std::convert::From<Self>,
    {
        self.into()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct I8(pub i8);
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct I16(pub i16);
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct I32(pub i32);
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct I64(pub i64);
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct I128(pub i128);

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct F32(pub f32);
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct F64(pub f64);

macro_rules! impl_number {
    ($($t:ty),+) => {
        $(impl Number for $t {})*
    };
}

// TODO: convert this to a proc_macro
macro_rules! impl_numfns {
    ($($t:ty),+) => {
        $(
            impl NumberFuncs for $t {
                #[inline]
                fn sqrt(self) -> Self {
                    Self(self.0.sqrt())
                }

                #[inline]
                fn cbrt(self) -> Self {
                    Self(self.0.cbrt())
                }

                #[inline]
                fn powi(self, n: i32) -> Self {
                    Self(self.0.powi(n))
                }

                #[inline]
                fn powf<Num: Number>(self, n: Num) -> Self {
                    // Self(self.0.powf(n));
                    panic!()
                }

                #[inline]
                fn exp(self) -> Self {
                    Self(self.0.exp())
                }

                #[inline]
                fn exp2(self) -> Self {
                    Self(self.0.exp2())
                }

                #[inline]
                fn ln(self) -> Self {
                    Self(self.0.ln())
                }

                #[inline]
                fn log(self, base: Self) -> Self {
                    Self(self.0.log(base.0))
                }

                #[inline]
                fn log2(self) -> Self {
                    Self(self.0.log2())
                }

                #[inline]
                fn log10(self) -> Self {
                    Self(self.0.log10())
                }

                #[inline]
                fn abs(self) -> Self {
                    Self(self.0.abs())
                }

                #[inline]
                fn signum(self) -> Self {
                    Self(self.0.signum())
                }

                #[inline]
                fn floor(self) -> Self {
                    Self(self.0.floor())
                }

                #[inline]
                fn ceil(self) -> Self {
                    Self(self.0.ceil())
                }

                #[inline]
                fn round(self) -> Self {
                    Self(self.0.round())
                }

                #[inline]
                fn trunc(self) -> Self {
                    Self(self.0.trunc())
                }

                #[inline]
                fn fract(self) -> Self {
                    Self(self.0.fract())
                }

                #[inline]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    Self(self.0.mul_add(a.0, b.0))
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    Self(self.0.max(other.0))
                }

                #[inline]
                fn min(self, other: Self) -> Self {
                    Self(self.0.min(other.0))
                }
            }
        )*
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

// #[allow(unused)]
// macro_rules! caster {
//     ($($tt:ty),+) => {
//         $(eval! {
//             let t = stringify!($tt).to_ascii_lowercase();
//             output! {
//                 impl From<{t}> for $tt {
//                     fn from(value: {t}) -> $tt {
//                         $tt(value)
//                     }
//                 }
//             }
//         })*
//     };
// }

// impls basic operators for each of the types (eg. +, -, /, *)
impl_number![F64, F32, I64, I32, I16, I8];
cast_number![F64, F32, I64, I32, I16, I8];
in_operator![F64, F32, I64, I32, I16, I8];
impl_numfns![F64, F32];
