// conrete_transform.rs
use super::{compute_flat_index, compute_strides, Transform, TransformError};
use crate::shape::{Shape, ShapeDescriptor};
use std::borrow::Cow;

// ======================= IdentityTransform =======================
///
/// A fundamental Transform that provides actions for describing behaviour
/// related to a tensors shape and indices
pub struct IdentityTransform(pub ShapeDescriptor);

impl Transform for IdentityTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        let strides = compute_strides(&self.0);
        compute_flat_index(logical, &strides)
    }

    fn shape(&self) -> &[usize] {
        &self.0
    }

    fn strides(&self) -> Cow<[usize]> {
        Cow::Owned(compute_strides(&self.0).into())
    }
}

// ======================= ChainedTransforms =======================
/// .
pub struct ChainedTransforms<'a> {
    pub stages: Vec<&'a dyn Transform>,
}

impl<'a> Transform for ChainedTransforms<'a> {
    fn to_flat(&self, logical: &[usize]) -> usize {
        todo!()
    }

    fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
        todo!()
    }

    fn shape(&self) -> &[usize] {
        todo!()
    }

    fn strides(&self) -> Cow<[usize]> {
        todo!()
    }
}

// ======================= ReshapeTransform =======================
/// .
pub struct ReshapeTransform {
    pub in_shape: ShapeDescriptor, // needed for computing to flat index from in_shape logical.
    pub out_shape: ShapeDescriptor, // needed for computing to out_shape logical from flat index.
    pub strides: Box<[usize]>, // only one set of strides since both volumes will always be the same.
}

impl ReshapeTransform {
    pub fn new(in_shape: ShapeDescriptor, out_shape: ShapeDescriptor) -> Result<Self, TransformError> {
        if in_shape.hypervolume() != out_shape.hypervolume() {
            return Err(TransformError::MisMatchHypervolume);
        }

        let strides = compute_strides(&in_shape);
        Ok(Self {
            in_shape,
            out_shape,
            strides,
        })
    }
}

impl Transform for ReshapeTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        todo!()
    }

    fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
        todo!()
    }

    fn shape(&self) -> &[usize] {
        &self.out_shape
    }

    fn strides(&self) -> Cow<[usize]> {
        Cow::Borrowed(&self.strides)
    }
}
