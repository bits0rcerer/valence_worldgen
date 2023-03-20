use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

pub struct CacheOnce(Box<dyn DensityFunction>);

impl CacheOnce {
    pub fn new(input: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(
            CacheOnce(input)
        )
    }
}

impl DensityFunction for CacheOnce {
    fn compute(&self, pos: BlockPos) -> f64 {
        // TODO: actually cache something here
        self.0.compute(pos)
    }

    fn map(&self, _: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        todo!()
    }

    fn min(&self) -> f64 {
        self.0.min()
    }

    fn max(&self) -> f64 {
        self.0.max()
    }
}