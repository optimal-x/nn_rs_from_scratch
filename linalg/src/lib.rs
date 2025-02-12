pub mod number;
pub mod vec_r2;
pub mod vec_r3;

pub mod ndarr {
    use crate::number::Number;
    use std::iter::Sum;
    use std::ops::*;
    use std::rc::Rc;

    pub trait ArrD<T: Number> {
        fn dot(&self, rhs: &Self) -> T;
        fn manhattan(&self, rhs: &Self) -> T; // Manhattan distance
        fn distance(&self, rhs: &Self) -> T; // Euclidean distance
    }

    /// Handle to 1D-array data.
    #[derive(Debug, Clone)]
    pub struct Arr1Handle<T: Number>(Rc<Vec<T>>);

    impl<T: Number> From<&Vec<T>> for Arr1Handle<T> {
        #[inline]
        fn from(value: &Vec<T>) -> Self {
            Self(Rc::new(value.to_vec()))
        }
    }

    /// Owner of 1D-array data.
    #[derive(Debug, Clone)]
    pub struct Arr1<T: Number>(Vec<T>);

    impl<T: Number> From<Vec<T>> for Arr1<T> {
        #[inline]
        fn from(value: Vec<T>) -> Self {
            Self(value)
        }
    }

    impl<T> ArrD<T> for Arr1<T>
    where
        T: Number + Sum + Copy,
        T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
    {
        fn dot(&self, rhs: &Self) -> T {
            assert_eq!(self.0.len(), rhs.0.len());
            self.0.iter().zip(rhs.0.iter()).map(|(r, l)| *r * *l).sum()
        }

        fn manhattan(&self, rhs: &Self) -> T {
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

        fn distance(&self, rhs: &Self) -> T {
            assert_eq!(self.0.len(), rhs.0.len());
            let s: T = self
                .0
                .iter()
                .zip(rhs.0.iter())
                .map(|(&r, &l)| (r - l) * (r - l))
                .sum();
            // s.sqrt();
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::number::I32;

    use super::vec_r2::Vec2;

    #[test]
    pub fn test_add() {
        let v1 = Vec2::new(I32(1), I32(2));
        let v2 = Vec2::new(I32(3), I32(4));
        assert_eq!(v1 + v2, Vec2::new(I32(4), I32(6)));
    }

    #[test]
    pub fn test_sub() {
        let v1 = Vec2::new(I32(1), I32(2));
        let v2 = Vec2::new(I32(3), I32(4));
        assert_eq!(v1 - v2, Vec2::new(I32(-2), I32(-2)));
    }

    #[test]
    pub fn test_mul() {
        let v1 = Vec2::new(I32(1), I32(2));
        let v2 = Vec2::new(I32(3), I32(4));
        assert_eq!(v1 * v2, I32(11));
    }
}
