use std::{borrow::Cow, ops::Deref};

use crate::ndarr::transform::default_slice;

// ======================= Shape =======================
pub trait Shape {
    /// the number of physical dimensions that the shape has
    fn rank(&self) -> usize;

    /// the actual structure of the shape required.
    fn shape(&self) -> Cow<ShapeDescriptor>;

    /// the total n-volume of a given shape, sometimes refered to as hypervolume.
    /// For example, in 5 dimensional shape we'd say it a has a 5-volume.
    fn hypervolume(&self) -> usize;

    fn compute_strides(&self) -> Box<[usize]> {
        let structure_shape = self.shape();
        let mut strides = default_slice(structure_shape.len());
        let mut stride = 1usize;

        for i in (0..structure_shape.len()).rev() {
            // store from back to front
            strides[i] = stride;
            stride *= structure_shape[i];
        }
        strides
    }
}

// ======================= ShapeDescriptor =======================
#[derive(Debug, PartialEq, Clone)]
pub struct ShapeDescriptor(pub Box<[usize]>);

/// the structure of a shape is infact a shape.
impl Shape for ShapeDescriptor {
    fn shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Borrowed(self)
    }

    #[inline]
    fn hypervolume(&self) -> usize {
        self.iter().product() // mul all the elements by each other.
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
