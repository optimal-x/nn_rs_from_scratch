use crate::shape::Shape;

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

pub fn compute_strides<const DIM: usize>(
    shape: &impl Shape<DIM>,
) -> Vec<usize> {
    let structure_shape = shape.shape();
    let mut strides = vec![0usize; structure_shape.len()];
    let mut stride = 1usize;

    for i in structure_shape.len() - 1..0 {
        // store from back to front
        strides[i] = stride;
        stride *= structure_shape[i];
    }
    strides
}

pub mod concrete_transformers {
    use super::Transform;

    pub struct ChainedTransforms<'a> {
        pub stages: Vec<&'a dyn Transform>,
    }

    /// pub-sub struct for watching transormation behaviour
    pub struct ReshapeTransform {
        pub in_shape: Vec<usize>,
        pub out_shape: Vec<usize>,
        pub strides: Vec<usize>,
    }
}

pub enum TransformError {
    ReshapeError,
    Error,
}

pub trait Transform {
    /// Maps a logical multi-dimensional index (e.g., [i, j, k]) to a flat index into the data buffer.
    fn to_flat(&self, logical: &[usize]) -> usize;

    /// Maps a flat index back to a logical index (e.g., [i, j, k]) if reversible.
    fn reverse(&self, flat_index: usize) -> Vec<usize>;

    /// Returns the logical shape after the transform.
    fn shape(&self) -> &[usize];

    /// Returns the logical strides used for index computation.
    fn strides(&self) -> &[usize];

    /// Optionally validates or adjusts the shape.
    fn reshape(&mut self) -> Result<(), TransformError>;
}
