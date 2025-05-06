// conrete_transform.rs
use super::{Transform, compute_flat_index, compute_strides};
use crate::shape::ShapeDescriptor;
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
    pub in_shape: ShapeDescriptor,
    pub out_shape: ShapeDescriptor,
    pub strides: Box<[usize]>, // only one set of strides since both volumes will always be the same
}

impl ReshapeTransform {
    pub fn new(
        in_shape: ShapeDescriptor,
        out_shape: ShapeDescriptor,
    ) -> Self {
        let strides = compute_strides(&out_shape);
        Self {
            in_shape,
            out_shape,
            strides 
        }
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
