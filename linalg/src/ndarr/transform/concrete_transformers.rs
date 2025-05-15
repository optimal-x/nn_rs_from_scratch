// conrete_transform.rs
use super::{
    Transform, TransformError, compute_flat_index, matching_hypervolume,
};
use crate::{ndarr::tensor::TensorAccess, shape::{Shape, ShapeDescriptor}};
use std::{borrow::Cow, ops::Index};

// ======================= IdentityTransform =======================
///
/// A fundamental Transform that provides actions for describing behaviour
/// related to a tensors shape and indices

type Strides = Box<[usize]>;

pub struct IdentityTransform(pub ShapeDescriptor, pub Strides);

impl Transform for IdentityTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        compute_flat_index(logical, &self.1)
    }

    fn out_shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Borrowed(&self.0)
    }

    fn out_strides(&self) -> Cow<[usize]> {
        Cow::Borrowed(&self.1)
    }
}

// ======================= ChainedTransforms =======================
/// .
pub struct ChainedTransforms<'a> {
    pub stages: Vec<&'a dyn Transform>,
}

// ======================= ChainedTransforms Transform =======================
impl Transform for ChainedTransforms<'_> {
    fn to_flat(&self, logical: &[usize]) -> usize {
        todo!()
    }

    fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
        todo!()
    }

    fn out_strides(&self) -> Cow<[usize]> {
        todo!()
    }

    fn out_shape(&self) -> Cow<ShapeDescriptor> {
        todo!()
    }
}

// ======================= ReshapeTransform =======================
/// .
pub struct ReshapeTransform {
    dst_shape: ShapeDescriptor,
    out_strides: Strides, // needed for computing to out_shape logical from flat index.
}

impl ReshapeTransform {
    pub fn new(
        src: &ShapeDescriptor,
        dst: ShapeDescriptor,
    ) -> Result<Self, TransformError> {
        assert!(
            matching_hypervolume(&dst, src).is_ok(),
            "Hypervolume Mismatch while reshaping"
        );
        let out_strides = dst.compute_strides();
        Ok(Self {
            dst_shape: dst,
            out_strides,
        })
    }
}

// ======================= ReshapeTransform Transform =======================
impl Transform for ReshapeTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        compute_flat_index(logical, &self.dst_shape)
    }

    fn out_shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Borrowed(&self.dst_shape)
    }

    fn out_strides(&self) -> Cow<[usize]> {
        Cow::Borrowed(&self.out_strides)
    }
}
