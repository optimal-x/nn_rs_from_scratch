use std::ops::Deref;

use crate::ndarr::transform::compute_strides;

// ======================= Shape =======================
pub trait Shape {
    /// the number of physical dimensions that the shape has
    fn rank(&self) -> usize;
    
    /// the actual structure of the shape required.
    fn shape(&self) -> ShapeDescriptor;
    
    /// the total n-volume of a given shape, sometimes refered to as hypervolume.
    /// For example, in 5 dimensional shape we'd say it a has a 5-volume.
    fn hypervolume(&self) -> usize;

    fn strides(&self) -> Box<[usize]> {
        compute_strides(&self.shape())
    }
}

// ======================= ShapeDescriptor =======================
#[derive(Debug, PartialEq, Clone)]
pub struct ShapeDescriptor(pub Box<[usize]>);

/// the structure of a shape is infact a shape.
impl Shape for ShapeDescriptor {
    fn shape(&self) -> ShapeDescriptor {
        self.clone()
    }

    #[inline]
    fn hypervolume(&self) -> usize {
        self.iter().fold(1, |acc, x| acc * x) // mul all the elements by each other.
    }

    #[inline]
    fn rank(&self) -> usize {
        self.len()
    }
}

// ======================= ShapeDescriptor Deref =======================
impl Deref for ShapeDescriptor {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ======================= ShapeDescriptor From<Box<[usize]>> =======================
impl From<Box<[usize]>> for ShapeDescriptor {
    fn from(value: Box<[usize]>) -> Self {
        Self(value)
    }
}
