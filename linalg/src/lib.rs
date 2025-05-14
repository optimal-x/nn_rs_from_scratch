pub mod ndarr;
pub mod number;
pub mod shape;
#[cfg(test)]
mod test;
pub mod vec_r2;
pub mod vec_r3;

#[macro_export]
macro_rules! slice {
    ($elem:expr; $n:expr) => (
        Box::new([$elem; $n])
    );
    ($($x:expr),+ $(,)?) => (
        Box::new([$($x),*])
    );
}
