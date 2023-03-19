use std::rc::Rc;

use valence::prelude::BlockPos;

use crate::density_function::{DensityFunction, sort_min_max};

pub struct Transformer<T: Fn(f64) -> f64> {
    f: Box<dyn DensityFunction>,
    transform: Rc<T>,
    min: f64,
    max: f64,
}

impl<T: Fn(f64) -> f64 + 'static> Transformer<T> {
    pub fn new(f: Box<dyn DensityFunction>, transform: Rc<T>) -> Box<dyn DensityFunction> {
        let (min, max) = sort_min_max(transform(f.min()), transform(f.max()));

        Box::new(Transformer {
            f,
            transform,
            min,
            max,
        })
    }
}

impl<T: Fn(f64) -> f64 + 'static> DensityFunction for Transformer<T> {
    fn compute(&self, pos: BlockPos) -> f64 {
        dbg!((self.transform)(self.f.compute(pos).abs()))
    }

    fn map(
        &self,
        visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        Transformer::new(self.f.map(visitor), self.transform.clone())
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}
