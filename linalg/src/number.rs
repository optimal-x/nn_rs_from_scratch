use std::{iter::Sum, ops::*};

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

macro_rules! impl_from_t {
    ($A:ty, $B:ty, $b:ty) => {
        impl From<$A> for $B {
            fn from(value: $A) -> Self {
                Self(value.0 as $b)
            }
        }
    };
}

impl_from_t!(F64, F32, f32);
impl_from_t!(F64, I32, i32);
impl_from_t!(F64, I64, i64);
impl_from_t!(F64, I16, i16);
impl_from_t!(F64, I8, i8);

impl_from_t!(F32, F64, f64);

impl_ops![F64, F32, I64, I32, I16, I8];
impl_number![F64, F32, I64, I32, I16, I8];
