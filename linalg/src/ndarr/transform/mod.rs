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
use crate::shape::{Shape, ShapeDescriptor};
use std::borrow::Cow;

// ======================= boxed_slice_from_fn_uninit =======================
/// .
pub fn slice_from_fn_uninit<T, F>(size: usize, f: F) -> Box<[T]>
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

// ======================= default_boxed_slice =======================
/// .
pub fn default_slice<T: Default>(size: usize) -> Box<[T]> {
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
#[deprecated]
pub fn compute_strides(shape: &impl Shape) -> Box<[usize]> {
    let structure_shape = shape.shape();
    let mut strides = default_slice(structure_shape.len());
    let mut stride = 1usize;

    for i in (0..structure_shape.len()).rev() {
        // store from back to front
        strides[i] = stride;
        stride *= structure_shape[i];
    }
    strides
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
pub fn matching_hypervolume(
    first: &impl Shape,
    second: &impl Shape,
) -> Result<(), TransformError> {
    if first.hypervolume() == second.hypervolume() {
        Ok(())
    } else {
        Err(TransformError::MisMatchHypervolume)
    }
}

// ======================= validate_permutation_and_shape =======================
/// .
/// # Errors
///
/// This function will return an error if:
/// - permutation length does not match the input shape length
/// - any of the permutations are out of bounds of the shape dimensions
/// - there is a duplicate index within the permutation
pub fn validate_permutation_and_shape(
    permutation: &[usize],
    input_shape: &[usize],
) -> Result<ShapeDescriptor, TransformError> {
    let rank = input_shape.len();

    if permutation.len() != rank {
        return Err(TransformError::Permute(PermutationError::WrongLength {
            expected: rank,
            actual: permutation.len(),
        }));
    }

    let mut seen = vec![false; rank];

    for &p in permutation {
        if p >= rank {
            return Err(TransformError::Permute(
                PermutationError::OutOfBounds { value: p, rank },
            ));
        }
        if seen[p] {
            return Err(TransformError::Permute(PermutationError::Duplicate {
                value: p,
            }));
        }
        seen[p] = true;
    }

    let output_shape: Box<[usize]> = permutation
        .iter()
        .map(|&i| input_shape[i])
        .collect::<Vec<usize>>()
        .into_boxed_slice();
    Ok(ShapeDescriptor(output_shape))
}

// ======================= TransformError =======================
/// .
#[derive(Debug)]
pub enum PermutationError {
    WrongLength { expected: usize, actual: usize },
    OutOfBounds { value: usize, rank: usize },
    Duplicate { value: usize },
}

#[derive(Debug)]
pub enum TransformError {
    ReshapeError,
    MisMatchHypervolume,
    Error,
    Permute(PermutationError)
}

// ======================= Transform =======================
/// .
pub trait Transform {
    /// Maps a logical multi-dimensional index (e.g., [i, j, k]) to a flat index into the data buffer.
    fn to_flat(&self, logical: &[usize]) -> usize;

    /// Maps a flat index back to a logical index (e.g., [i, j, k]) if reversible.
    fn to_logical(&self, flat_index: usize) -> Box<[usize]> {
        let strides = self.out_strides();
        let shape = self.out_shape();
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
    fn out_shape(&self) -> Cow<ShapeDescriptor>;

    /// Returns the logical strides used for index computation.
    fn out_strides(&self) -> Cow<[usize]>;
    
}

// ======================= Transform =======================
/// .
pub trait IndexAccess<T> {
    fn get<'a>(&self, buffer: &'a [T], logical: &[usize]) -> Option<&'a T>;
    fn get_mut<'a>(&self, buffer: &'a mut [T], logical: &[usize]) -> Option<&'a mut T>;
}

// Generic implementation of IndexAccess for all types that impl transform
impl<T, U: Transform> IndexAccess<T> for U {
    fn get<'a>(&self, buffer: &'a [T], logical: &[usize]) -> Option<&'a T> {
        buffer.get(self.to_flat(logical))
    }

    fn get_mut<'a>(&self, buffer: &'a mut [T], logical: &[usize]) -> Option<&'a mut T> {
        let idx = self.to_flat(logical);
        buffer.get_mut(idx)
    }
}
