use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn quarter_negative(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, |x| 0.25 * (-v))
}
