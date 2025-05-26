use std::borrow::Cow;

use crate::{ndarr::{X, Y, Z}, shape::{Shape, ShapeDescriptor}};

///====================== Vec3 ======================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T>([T; 3]);
impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

///====================== Vec3 Shape ======================
impl<T> Shape for Vec3<T> {
    fn shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Owned(ShapeDescriptor(Box::new([3])))
    }

    fn hypervolume(&self) -> usize {
        3
    }

    fn rank(&self) -> usize {
        1
    }
}

///====================== Vec3 Neg ======================
impl<T> std::ops::Neg for Vec3<T>
where
    T: Clone + Copy,
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self[X], -self[Y], -self[Z])
    }
}

///====================== Vec3 Add ======================
impl<T> std::ops::Add for Vec3<T>
where
    T: std::ops::Add<Output = T>,
    T: Clone + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x = self[X] + rhs[X];
        let new_y = self[Y] + rhs[Y];
        let new_z = self[Z] + rhs[Z];
        Vec3::new(new_x, new_y, new_z)
    }
}

///====================== Vec3 Mul ======================
impl<T> std::ops::Mul for Vec3<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Clone + Copy,
{
    type Output = T;

    /// DOT product
    fn mul(self, rhs: Self) -> Self::Output {
        let new_x = self[X] * rhs[X];
        let new_y = self[Y] * rhs[Y];
        let new_z = self[Z] * rhs[Z];
        new_x + new_y + new_z
    }
}

///====================== Vec3 Sub ======================
impl<T> std::ops::Sub for Vec3<T>
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
impl<T> std::ops::Index<X> for Vec3<T> {
    type Output = T;

    fn index(&self, _index: X) -> &Self::Output {
        &self.0[0]
    }
}

impl<T> std::ops::Index<Y> for Vec3<T> {
    type Output = T;

    fn index(&self, _index: Y) -> &Self::Output {
        &self.0[1]
    }
}

impl<T> std::ops::Index<Z> for Vec3<T> {
    type Output = T;

    fn index(&self, _index: Z) -> &Self::Output {
        &self.0[2]
    }
}

//====================== Vec2 IndexMut ======================
impl<T> std::ops::IndexMut<X> for Vec3<T> {
    fn index_mut(&mut self, _index: X) -> &mut Self::Output {
        &mut self.0[0]
    }
}

impl<T> std::ops::IndexMut<Y> for Vec3<T> {
    fn index_mut(&mut self, _index: Y) -> &mut Self::Output {
        &mut self.0[1]
    }
}

impl<T> std::ops::IndexMut<Z> for Vec3<T> {
    fn index_mut(&mut self, _index: Z) -> &mut Self::Output {
        &mut self.0[2]
    }
}
