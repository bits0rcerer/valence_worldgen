use std::sync::Arc;

use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn squeeze(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, Arc::new(|x: f64| {
        let clamped = x.clamp(-1.0, 1.0);
        (clamped / 2.0) - (clamped.powi(3) / 24.0)
    }))
}
