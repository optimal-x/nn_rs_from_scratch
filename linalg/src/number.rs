use cast_from_list::casting_number;
use std::{iter::Sum, ops::*};

pub trait NumberFuncs {
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
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

macro_rules! impl_num_funcs {
    ($($t:ty),+) => {
        $(
            impl NumberFuncs for $t {
                fn sqrt(self) -> Self {
                    Self(self.0.sqrt())
                }

                fn cbrt(self) -> Self {
                    Self(self.0.cbrt())
                }

                fn powi(self, n: i32) -> Self {
                    Self(self.0.powi(n))
                }

                fn powf(self, n: Self) -> Self {
                    Self(self.0.powf(n.0))
                }

                fn exp(self) -> Self {
                    Self(self.0.exp())
                }

                fn exp2(self) -> Self {
                    Self(self.0.exp2())
                }

                fn ln(self) -> Self {
                    Self(self.0.ln())
                }

                fn log(self, base: Self) -> Self {
                    Self(self.0.log(base.0))
                }

                fn log2(self) -> Self {
                    Self(self.0.log2())
                }

                fn log10(self) -> Self {
                    Self(self.0.log10())
                }

                fn abs(self) -> Self {
                    Self(self.0.abs())
                }

                fn signum(self) -> Self {
                    Self(self.0.signum())
                }

                fn floor(self) -> Self {
                    Self(self.0.floor())
                }

                fn ceil(self) -> Self {
                    Self(self.0.ceil())
                }

                fn round(self) -> Self {
                    Self(self.0.round())
                }

                fn trunc(self) -> Self {
                    Self(self.0.trunc())
                }

                fn fract(self) -> Self {
                    Self(self.0.fract())
                }

                fn mul_add(self, a: Self, b: Self) -> Self {
                    Self(self.0.mul_add(a.0, b.0))
                }

                fn max(self, other: Self) -> Self {
                    Self(self.0.max(other.0))
                }

                fn min(self, other: Self) -> Self {
                    Self(self.0.min(other.0))
                }
            }
        )*
    };
}

macro_rules! impl_ops {
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

// Proc Macro that enables casting between number types
casting_number!(F64, F32, I64, I32, I16, I8);

// impls basic operators for each of the types (eg. +, -, /, *)
impl_ops![F64, F32, I64, I32, I16, I8];

// impls the number trait
impl_number![F64, F32, I64, I32, I16, I8];

// impls for number functions
impl_num_funcs![F64, F32];
