use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

pub struct Cache2D(Box<dyn DensityFunction>);

impl Cache2D {
    pub fn new(input: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(
            Cache2D(input)
        )
    }
}

impl DensityFunction for Cache2D {
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