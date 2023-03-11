use crate::density_function::commutative::{Commutative, Operation};
use crate::density_function::DensityFunction;

pub fn max(f1: Box<dyn DensityFunction>, f2: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Commutative::new(f1, f2, Operation::Max)
}