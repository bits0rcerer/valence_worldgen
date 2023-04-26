use valence_core::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};

pub struct FlatCache(Box<dyn DensityFunction>);

impl FlatCache {
    pub fn new(input: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(FlatCache(input))
    }
}

impl DensityFunction for FlatCache {
    fn compute(&self, pos: BlockPos) -> f64 {
        // TODO: actually cache something here
        self.0.compute(BlockPos::new(pos.x, 0, pos.z))
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        context_provider.fill_direct(slice, self)
    }

    fn min(&self) -> f64 {
        self.0.min()
    }

    fn max(&self) -> f64 {
        self.0.max()
    }
}
