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

pub fn flatten_indices<const DIM: usize>(
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

    return strides;
}
