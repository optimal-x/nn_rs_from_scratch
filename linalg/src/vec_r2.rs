use std::borrow::Cow;

use crate::{ndarr::{X, Y}, shape::{Shape, ShapeDescriptor}};

//====================== Vec2 ======================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T>([T; 2]);
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}

//====================== Vec2 Shape ======================
impl<T> Shape for Vec2<T> {
    #[inline(always)]
    fn shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Owned(ShapeDescriptor(Box::new([2])))
    }

    #[inline(always)]
    fn hypervolume(&self) -> usize {
        2
    }

    #[inline(always)]
    fn rank(&self) -> usize {
        1
    }
}

//====================== Vec2 Neg ======================
impl<T> std::ops::Neg for Vec2<T>
where
    T: Clone + Copy,
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.0[0], -self.0[1])
    }
}

//====================== Vec2 Add ======================
impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
    T: Clone + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x = self.0[0] + rhs.0[0];
        let new_y = self.0[1] + rhs.0[1];
        Vec2::new(new_x, new_y)
    }
}

//====================== Vec2 Mul ======================
impl<T> std::ops::Mul for Vec2<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Clone + Copy,
{
    type Output = T;

    /// DOT product
    fn mul(self, rhs: Self) -> Self::Output {
        let new_x = self.0[0] * rhs.0[0];
        let new_y = self.0[1] * rhs.0[1];
        new_x + new_y
    }
}

//====================== Vec2 Sub ======================
impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Add<Output = T>,
    T: std::ops::Neg<Output = T>,
    T: Clone + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

//====================== Vec2 Index ======================
impl<T> std::ops::Index<X> for Vec2<T> {
    type Output = T;

    fn index(&self, _index: X) -> &Self::Output {
        &self.0[0]
    }
}

impl<T> std::ops::Index<Y> for Vec2<T> {
    type Output = T;

    fn index(&self, _index: Y) -> &Self::Output {
        &self.0[1]
    }
}

//====================== Vec2 IndexMut ======================
impl<T> std::ops::IndexMut<X> for Vec2<T> {
    fn index_mut(&mut self, _index: X) -> &mut Self::Output {
        &mut self.0[0]
    }
}

impl<T> std::ops::IndexMut<Y> for Vec2<T> {
    fn index_mut(&mut self, _index: Y) -> &mut Self::Output {
        &mut self.0[1]
    }
}
