use std::ops::Deref;

///.
#[derive(Debug, PartialEq)]
pub struct Shape<const DIMS: usize>(pub [usize; DIMS]);

impl<const DIMS: usize> Deref for Shape<DIMS> {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIMS: usize> From<[usize; DIMS]> for Shape<DIMS> {
    fn from(value: [usize; DIMS]) -> Self {
        Self(value)
    }
}
