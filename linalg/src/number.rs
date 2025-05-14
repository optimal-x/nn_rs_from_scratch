#![allow(dead_code)]

pub trait NaturalFuncs<T> {
    fn abs(self) -> T;
    fn signum(self) -> T;
}

pub trait RealFuncs<T>: NaturalFuncs<T> {
    fn sqrt(self) -> T;
    fn cbrt(self) -> T;
    fn powi(self, n: i32) -> T;
    fn powf(self, n: T) -> T;
    fn exp(self) -> T;
    fn exp2(self) -> T; //2^x
    fn ln(self) -> T;
    fn log(self, base: T) -> T;
    fn log2(self) -> T;
    fn log10(self) -> T;
    fn floor(self) -> T;
    fn ceil(self) -> T;
    fn round(self) -> T;
    fn trunc(self) -> T;
    fn fract(self) -> T;
    fn mul_add(self, a: T, b: T) -> T;
    fn max(self, other: T) -> T;
    fn min(self, other: T) -> T;
}

macro_rules! impl_natural {
    ($($tt:ty),+) => {
        $(
            impl NaturalFuncs<$tt> for $tt {
                fn abs(self) -> $tt {
                    <$tt>::abs(self)
                }

                fn signum(self) -> $tt {
                    <$tt>::signum(self)
                }
            }
        )*
    }
}

macro_rules! impl_reals {
    ($($tt:ty),+) => {
        $(
            impl RealFuncs<$tt> for $tt {
                fn sqrt(self) -> $tt {
                    <$tt>::sqrt(self)
                }

                fn cbrt(self) -> $tt {
                    <$tt>::cbrt(self)
                }

                fn powi(self, n: i32) -> $tt {
                    <$tt>::powi(self, n)
                }

                fn powf(self, n: f64) -> $tt {
                    <$tt>::powf(self, n)
                }

                fn exp(self) -> $tt {
                    <$tt>::exp(self)
                }

                fn exp2(self) -> $tt {
                    <$tt>::exp2(self)
                }

                fn ln(self) -> $tt {
                    <$tt>::ln(self)
                }

                fn log(self, base: f64) -> $tt {
                    <$tt>::log(self, base)
                }

                fn log2(self) -> $tt {
                    <$tt>::log2(self)
                }

                fn log10(self) -> $tt {
                    <$tt>::log10(self)
                }

                fn floor(self) -> $tt {
                    <$tt>::floor(self)
                }

                fn ceil(self) -> $tt {
                    <$tt>::ceil(self)
                }

                fn round(self) -> $tt {
                    <$tt>::round(self)
                }

                fn trunc(self) -> $tt {
                    <$tt>::trunc(self)
                }

                fn fract(self) -> $tt {
                    <$tt>::fract(self)
                }

                fn mul_add(self, a: $tt, b: $tt) -> $tt {
                    <$tt>::mul_add(self, a, b)
                }

                fn max(self, other: $tt) -> $tt {
                    <$tt>::max(self, other)
                }

                fn min(self, other: $tt) -> $tt {
                    <$tt>::min(self, other)
                }
            }
        )*
    };
}

macro_rules! impl_real_and_natural {
    ($($tt:ty),+) => {
        $(
            impl_natural!($tt);
            impl_reals!($tt);
        )*
    }
}

// both
impl_real_and_natural![f64];

// natural only
impl_natural![i32, i64, i128];
