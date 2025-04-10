use crate::{ndarr::tensor::Container, shape::{Shape, StructureShape}};

///====================== Vec2 ======================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T>([T; 2]);
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}

///====================== Vec2 Shape ======================
impl<T> Shape<1> for Vec2<T> {
    #[inline(always)]
    fn shape(&self) -> StructureShape<1> {
        StructureShape::<1>::from([2])
    }

    #[inline(always)]
    fn n_volume(&self) -> usize {
        2
    }

}

///====================== Vec2 Container ======================
impl<T> Container<T, 1> for Vec2<T> {
    fn at(&self, indicies: &[usize]) -> Option<&T> {
        assert_eq!(indicies.len(), Self::RANK);
        todo!() // TODO
    }

    fn set_at(&self, indicies: &[usize], value: T) {
        assert_eq!(indicies.len(), Self::RANK);
        todo!() // TODO
    }
}

///====================== Vec2 Neg ======================
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

///====================== Vec2 Add ======================
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

///====================== Vec2 Mul ======================
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

///====================== Vec2 Sub ======================
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
