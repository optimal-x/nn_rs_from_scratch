#![allow(dead_code)]

pub trait GetSetDelegate {
    fn get(&self) -> Box<dyn GetSetDelegate>;
    fn set(&self, value: Box<dyn GetSetDelegate>);
}

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
