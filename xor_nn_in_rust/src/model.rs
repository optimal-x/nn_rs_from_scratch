pub trait ActivationFunction {
    type OutputZ;
    fn r#in() -> Self::OutputZ;
    fn r#out() -> Self::OutputZ;
}

struct ReLU;
impl ActivationFunction for ReLU {
    type OutputZ = f64;

    fn r#in() -> Self::OutputZ {
        todo!()
    }

    fn r#out() -> Self::OutputZ {
        todo!()
    }
}
