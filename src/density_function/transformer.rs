use std::sync::Arc;

use valence_protocol::block_pos::BlockPos;

use crate::density_function::{sort_min_max, DensityFunction, ContextProvider};

pub struct Transformer<T: Fn(f64) -> f64 + Sync> {
    f: Box<dyn DensityFunction>,
    transform: Arc<T>,
    min: f64,
    max: f64,
}

impl<T: Fn(f64) -> f64 + 'static + Sync + Send> Transformer<T> {
    pub fn new(f: Box<dyn DensityFunction>, transform: Arc<T>) -> Box<dyn DensityFunction> {
        let (min, max) = sort_min_max(transform(f.min()), transform(f.max()));

        Box::new(Transformer {
            f,
            transform,
            min,
            max,
        })
    }
}

impl<T: Fn(f64) -> f64 + 'static + Sync + Send> DensityFunction for Transformer<T> {
    fn compute(&self, pos: BlockPos) -> f64 {
        (self.transform)(self.f.compute(pos))
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        self.f.fill(slice, context_provider);
        slice.iter_mut().for_each(|v| *v = (self.transform)(*v))
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}
