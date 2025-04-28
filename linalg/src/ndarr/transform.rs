use std::borrow::Cow;

/// reshape.rs
/// ===========
/// I want to try to make this reshape module work by keeping the initial shape of the
/// Tensor intact but instead giving the tensor new mapping s.t. when we index as if
/// the Tensor was reshaped it will map to the "reshaped" indicies into that of the
/// originial shape.
///
/// Example:
/// ```rs
/// // some 2x3 tensor
/// // [[0 0 0] [0 0 0]]
/// let mut tensor = Tensor::<2>::new(vec![vec![0; 3]; 2]);
///
/// // reshaped mapping into:
/// // [[[0 0 0] [0 0 0]]]
/// // but in memory its still the same.
/// tensor.reshape(StructureShape::<3>::from([1,2,3]));
/// ```
use crate::shape::Shape;

/// compute_strides
pub fn compute_strides(shape: &impl Shape) -> Box<[usize]> {
    let structure_shape = shape.shape();
    let mut strides = vec![0usize; structure_shape.len()];
    let mut stride = 1usize;

    for i in (0..structure_shape.len()).rev() {
        // store from back to front
        strides[i] = stride;
        stride *= structure_shape[i];
    }
    strides.into()
}

pub fn compute_flat_index(logical: &[usize], strides: &[usize]) -> usize {
    logical
        .iter()
        .zip(strides.iter())
        .map(|(idx, stride)| idx * stride)
        .sum()
}

pub fn matching_hypervolume<'a>(
    first: &impl Shape,
    second: &impl Shape,
) -> Result<(), TransformError> {
    if first.hypervolume() == second.hypervolume() {
        Ok(())
    } else {
        Err(TransformError::MisMatchHypervolume)
    }
}

pub mod concrete_transformers {
    use super::{Transform, compute_flat_index, compute_strides};
    use crate::shape::ShapeDescriptor;
    use std::borrow::Cow;

    /// A fundamental Transform that provides actions for describing behaviour
    /// related to a tensors shape and indices
    pub struct IdentityTransform(pub ShapeDescriptor);

    impl Transform for IdentityTransform {
        fn to_flat(&self, logical: &[usize]) -> usize {
            let strides = compute_strides(&self.0);
            compute_flat_index(logical, &strides)
        }

        fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
            let strides = self.strides();
            let shape = self.shape();
            let rank = strides.len();

            let mut uninit_box = Box::<[usize]>::new_uninit_slice(rank);
            for i in 0..rank {
                let stride = strides[i];
                let axis = shape[i];

                let logical_idx = (flat_index / stride) % axis;
                uninit_box[i].write(logical_idx);
            }
            unsafe { uninit_box.assume_init() }
        }

        fn shape(&self) -> &[usize] {
            &self.0
        }

        fn strides(&self) -> Cow<[usize]> {
            Cow::Owned(compute_strides(&self.0).into())
        }
    }

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

    pub struct ReshapeTransform {
        pub in_shape: ShapeDescriptor,
        pub out_shape: ShapeDescriptor,
        pub strides: Box<[usize]>, // only one set of strides since both volumes will always be the same
    }

    impl Transform for ReshapeTransform {
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
}

pub enum TransformError {
    ReshapeError,
    MisMatchHypervolume,
    Error,
}

pub trait Transform {
    /// Maps a logical multi-dimensional index (e.g., [i, j, k]) to a flat index into the data buffer.
    fn to_flat(&self, logical: &[usize]) -> usize;

    /// Maps a flat index back to a logical index (e.g., [i, j, k]) if reversible.
    fn to_logical(&self, flat_index: usize) -> Box<[usize]>;

    /// Returns the logical shape after the transform.
    fn shape(&self) -> &[usize];

    /// Returns the logical strides used for index computation.
    fn strides(&self) -> Cow<[usize]>;
}
