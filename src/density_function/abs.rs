use std::rc::Rc;

use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn abs(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, Rc::new(|x: f64| x.abs()))
}
