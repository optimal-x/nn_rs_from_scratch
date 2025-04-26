use std::ops::Deref;

pub trait Shape {
    fn rank(&self) -> usize;
    
    /// the actual structure of the shape required.
    fn shape(&self) -> StructureShape;
    
    /// the total n-volume of a given shape, sometimes refered to as hypervolume.
    /// For example, in 5 dimensional shape we'd say it a has a 5-volume.
    fn n_volume(&self) -> usize;
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructureShape(pub Box<[usize]>);

/// the structure of a shape is infact a shape.
impl Shape for StructureShape {
    fn shape(&self) -> StructureShape {
        self.clone()
    }

    fn n_volume(&self) -> usize {
        self.iter().fold(1, |acc, x| acc * x) // mul all the elements by each other.
    }

    fn rank(&self) -> usize {
        todo!()
    }
}

impl Deref for StructureShape {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Box<[usize]>> for StructureShape {
    fn from(value: Box<[usize]>) -> Self {
        Self(value)
    }
}
