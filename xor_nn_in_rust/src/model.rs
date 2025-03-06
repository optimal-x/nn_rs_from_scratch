use linalg::ndarr::arr2::Arr2;
use linalg::number::*;

pub trait ActivationFunction {
    type OutputZ;
    fn a() -> Self::OutputZ;
    fn prime() -> Self::OutputZ;
}

struct ReLU;
impl ActivationFunction for ReLU {
    type OutputZ = F64;

    fn a() -> Self::OutputZ {
        // In our case Z is a vector of weight-bias input-outputs
        //
        // # in other words
        // max(Z, 0)
        todo!()
    }

    fn prime() -> Self::OutputZ {
        todo!()
    }
}
