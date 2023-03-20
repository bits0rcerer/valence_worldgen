use std::sync::Arc;

use crate::density_function::transformer::Transformer;
use crate::density_function::DensityFunction;

pub fn abs(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, Arc::new(|x: f64| x.abs()))
}
