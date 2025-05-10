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
pub struct ReshapeTransform<'a> {
    src_shape: &'a ShapeDescriptor,
    dst_shape: ShapeDescriptor,
    strides: Box<[usize]>, // needed for computing to out_shape logical from flat index.
}

impl<'a> ReshapeTransform<'a> {
    pub fn new(src: &'a ShapeDescriptor, dst: ShapeDescriptor) -> Result<Self, TransformError> {
        let strides = src.strides();
        Ok(Self {
            src_shape: src,
            dst_shape: dst,
            strides
        })
    }
}

// ======================= ReshapeTransform Transform =======================
impl<'a> Transform for ReshapeTransform<'a> {
    fn to_flat(&self, logical: &[usize]) -> usize {
        todo!()
    }

    fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
        todo!()
    }

    fn shape(&self) -> &[usize] {
        &self.dst_shape
    }

    fn strides(&self) -> Cow<[usize]> {
        Cow::Borrowed(&self.strides)
    }
}
