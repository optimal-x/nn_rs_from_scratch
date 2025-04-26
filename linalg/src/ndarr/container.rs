use crate::shape::Shape;

pub trait Container<T>: Shape {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&mut self, indicies: &[usize], value: T);
}
