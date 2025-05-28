// conrete_transform.rs
use super::{
    compute_flat_index, matching_hypervolume, validate_permutation_and_shape, PermutationError, Transform, TransformError
};
use crate::shape::{Shape, ShapeDescriptor};
use std::borrow::Cow;

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

// ======================= ReshapeTransform =======================
/// .
pub struct ReshapeTransform {
    out_shape: ShapeDescriptor,
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
            out_shape: dst,
            out_strides,
        })
    }
}

// ======================= ReshapeTransform Transform =======================
impl Transform for ReshapeTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        compute_flat_index(logical, &self.out_shape)
    }

    fn out_shape(&self) -> Cow<ShapeDescriptor> {
        Cow::Borrowed(&self.out_shape)
    }

    fn out_strides(&self) -> Cow<[usize]> {
        Cow::Borrowed(&self.out_strides)
    }
}

// ======================= TransposeTransform =======================
pub struct TransposeTransform {
    reshape: ReshapeTransform,
    permutations: Box<[usize]>,
}


impl TransposeTransform {
    pub fn new(
        src: &ShapeDescriptor,
        permutation: Box<[usize]>,
    ) -> Result<Self, TransformError> {
        let output_shape = validate_permutation_and_shape(&permutation, &src)?;

        Ok(Self {
            reshape: ReshapeTransform::new(src, output_shape)?,
            permutations: permutation,
        })
    }
}

// ======================= TransposeTransform Transform =======================
impl Transform for TransposeTransform {
    fn to_flat(&self, logical: &[usize]) -> usize {
        todo!()
    }

    fn out_shape(&self) -> Cow<ShapeDescriptor> {
        todo!()
    }

    fn out_strides(&self) -> Cow<[usize]> {
        todo!()
    }
}
