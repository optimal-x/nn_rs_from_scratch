use crate::shape::Shape;

pub trait Container<T>: Shape {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&self, indicies: &[usize], value: T);
}
