use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn squeeze(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, |x| {
        let clamped = v.clamp(-1.0, 1.0);
        (clamped / 2.0) - (clamped.powi(3) / 24.0)
    })
}
