use crate::shape::Shape;

pub trait Container<T, const DIM: usize>: Shape<DIM> {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&self, indicies: &[usize], value: T);
}
