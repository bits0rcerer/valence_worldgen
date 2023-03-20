use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

pub struct FlatCache(Box<dyn DensityFunction>);

impl FlatCache {
    pub fn new(input: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(
            FlatCache(input)
        )
    }
}

impl DensityFunction for FlatCache {
    fn compute(&self, pos: BlockPos) -> f64 {
        // TODO: actually cache something here
        self.0.compute(BlockPos::new(pos.x, 0, pos.z))
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