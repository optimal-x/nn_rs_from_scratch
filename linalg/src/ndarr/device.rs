use super::tensor::Tensor;

pub enum DeviceError {
    HandleError
}

// ======================= Device =======================
pub trait Device<'a> {
    fn upload<T>(&self, tensor: &Tensor<T>) -> Result<DeviceTensorHandle<T>, DeviceError>;
    fn launch_kernel();
    fn download<T>(&self, handle: &DeviceTensorHandle<T>) -> Result<Tensor<T>, DeviceTensorHandle<T>>;
}

// ======================= DeviceBackend =======================
pub struct DeviceTensorHandle<T> {
    temp: T // TODO remove and figure out a good use for the type LOL
}

#[derive(Default)]
pub enum DeviceBackend {
    #[default]
    CPU,
    CUDA,
    VULKAN,
    WEBGL,
}

// pub mod slice_cpu {
//     use crate::ndarr::tensor::Tensor;
//     use super::Device;

//     impl<T> Device<T> for Box<[T]> {
//         type Idx = usize;

//         fn at(&self, flat_index: Self::Idx) -> Option<&T> {
//             if 0 >= flat_index && flat_index < self.len() {
//                 Some(&self[flat_index])
//             } else {
//                 panic!("flat indexing out of bounds");
//             }
//         }

//         fn set_at(&mut self, flat_index: Self::Idx, value: T) {
//             if 0 >= flat_index && flat_index < self.len() {
//                 self[flat_index] = value;
//             } else {
//                 panic!("flat indexing out of bounds");
//             }
//         }
//     }

//     impl<'a, T> From<Box<[T]>> for Tensor<'a, T, Box<[T]>> {
//         fn from(value: Box<[T]>) -> Self {
//             Tensor::new(value)
//         }
//     }
// }
