pub mod concrete_transformers;

/// reshape.rs
/// ===========
/// I want to try to make this reshape module work by keeping the initial shape of the
/// Tensor intact but instead giving the tensor new mapping s.t. when we index as if
/// the Tensor was reshaped it will map to the "reshaped" indicies into that of the
/// originial shape.
///
/// Example:
/// ```rs
/// // some 2x3 tensor, but its contiguous
/// // [[0 0 0] [0 0 0]]
/// let mut tensor = Tensor::<2>::new(vec![vec![0; 3]; 2]);
///
/// // reshaped mapping into:
/// // [[[0 0 0] [0 0 0]]]
/// // but in memory its still the same.
/// tensor.reshape(StructureShape::<3>::from([1,2,3]));
/// ```
use crate::shape::Shape;
use std::borrow::Cow;

// ======================= boxed_slice_from_fn_uninit =======================
///.
fn boxed_slice_from_fn<T, F>(size: usize, f: F) -> Box<[T]>
where
    F: Fn(usize) -> T,
{
    (0..size).map(f).collect::<Vec<T>>().into_boxed_slice()
}

// ======================= boxed_slice_from_fn_uninit =======================
/// .
pub fn boxed_slice_from_fn_uninit<T, F>(size: usize, f: F) -> Box<[T]>
where
    F: Fn(usize) -> T,
{
    let mut boxed_uninit = Box::<[T]>::new_uninit_slice(size);
    for (idx, slot) in boxed_uninit.iter_mut().enumerate() {
        slot.write(f(idx));
    }

    // SAFETY: All elements are initialized above, so it's safe to assume_init
    unsafe { boxed_uninit.assume_init() }
}

pub fn default_boxed_slice<T: Default>(size: usize) -> Box<[T]> {
    let mut boxed_uninit = Box::<[T]>::new_uninit_slice(size);
    boxed_uninit.iter_mut().for_each(|slot| {
        slot.write(T::default());
    });

    // SAFETY: All elements are initialized above because of default,
    // so it's safe to assume_init
    unsafe { boxed_uninit.assume_init() }
}

// ======================= compute_strides =======================
/// .
pub fn compute_strides(shape: &impl Shape) -> Box<[usize]> {
    let structure_shape = shape.shape();
    let mut strides = default_boxed_slice(structure_shape.len());
    let mut stride = 1usize;

    for i in (0..structure_shape.len()).rev() {
        // store from back to front
        strides[i] = stride;
        stride *= structure_shape[i];
    }
    strides.into()
}

// ======================= compute_flat_index =======================
/// .
pub fn compute_flat_index(logical: &[usize], strides: &[usize]) -> usize {
    logical
        .iter()
        .zip(strides.iter())
        .map(|(idx, stride)| idx * stride)
        .sum()
}

// ======================= matching_hypervolume =======================
/// .
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

// ======================= TransformError =======================
/// .
pub enum TransformError {
    ReshapeError,
    MisMatchHypervolume,
    Error,
}

// ======================= Transform =======================
/// .
pub trait Transform {
    /// Maps a logical multi-dimensional index (e.g., [i, j, k]) to a flat index into the data buffer.
    fn to_flat(&self, logical: &[usize]) -> usize;

    /// Maps a flat index back to a logical index (e.g., [i, j, k]) if reversible.
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

    /// Returns the logical shape after the transform.
    fn shape(&self) -> &[usize];

    /// Returns the logical strides used for index computation.
    fn strides(&self) -> Cow<[usize]>;
}
