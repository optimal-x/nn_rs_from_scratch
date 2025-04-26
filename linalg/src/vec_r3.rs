use crate::shape::{Shape, StructureShape};

///====================== Vec3 ======================
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>([T; 3]);
impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}

///====================== Vec3 Shape ======================
impl<T> Shape for Vec3<T> {
    fn shape(&self) -> StructureShape {
        StructureShape::from(vec![3].into_boxed_slice())
    }

    fn n_volume(&self) -> usize {
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
        Self::new(-self.0[0], -self.0[1], -self.0[2])
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
        let new_x = self.0[0] + rhs.0[0];
        let new_y = self.0[1] + rhs.0[1];
        let new_z = self.0[2] + rhs.0[2];
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
        let new_x = self.0[0] * rhs.0[0];
        let new_y = self.0[1] * rhs.0[1];
        let new_z = self.0[2] * rhs.0[2];
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
