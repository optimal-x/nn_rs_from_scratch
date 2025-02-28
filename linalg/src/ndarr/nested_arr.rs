use super::arr1::Arr1;

/// Wouldn't heavily recommend using this since caching for this is probably really bad
#[derive(Debug)]
pub enum NestedArr<T> {
    Elem(T),
    Vec(Arr1<NestedArr<T>>),
}

impl<T> NestedArr<T> {
    pub fn dims(&self) -> usize {
        match self {
            NestedArr::Elem(_) => 1,
            NestedArr::Vec(sub_arr) => 1 + deeper(sub_arr),
        }
    }
}

pub(self) fn deeper<T>(sub_arr: &Arr1<NestedArr<T>>) -> usize {
    if let NestedArr::Vec(deeper_sub_arr) = &sub_arr[0] {
        1 + deeper(deeper_sub_arr)
    } else {
        1
    }
}
