use std::ops::Deref;

pub trait Shape<const DIM: usize> {
    const RANK: usize = DIM;
    /// the actual structure of the shape required.
    fn shape(&self) -> StructureShape<DIM>;
    
    /// the total n-volume of a given shape, sometimes refered to as hypervolume.
    /// For example, in 5 dimensional shape we'd say it a has a 5-volume.
    fn n_volume(&self) -> usize;
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructureShape<const DIM: usize>(pub [usize; DIM]);

/// the structure of a shape is infact a shape.
impl<const DIM: usize> Shape<DIM> for StructureShape<DIM> {
    fn shape(&self) -> StructureShape<DIM> {
        self.clone()
    }

    fn n_volume(&self) -> usize {
        self.iter().fold(1, |acc, x| acc * x) // mul all the elements by each other.
    }
}


impl<const DIM: usize> Deref for StructureShape<DIM> {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIM: usize> From<[usize; DIM]> for StructureShape<DIM> {
    fn from(value: [usize; DIM]) -> Self {
        Self(value)
    }
}
