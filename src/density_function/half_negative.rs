use std::rc::Rc;

use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn half_negative(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, Rc::new(|x: f64| 0.5 * (-x)))
}
