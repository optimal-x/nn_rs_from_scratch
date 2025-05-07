// ======================= Container =======================
pub trait Device<T> {
    fn at(&self, indicies: &[usize]) -> Option<&T>;
    fn set_at(&mut self, indicies: &[usize], value: T);
}
