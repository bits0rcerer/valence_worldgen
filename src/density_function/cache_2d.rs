use valence_protocol::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};

pub struct Cache2D(Box<dyn DensityFunction>);

impl Cache2D {
    pub fn new(input: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(Cache2D(input))
    }
}

impl DensityFunction for Cache2D {
    fn compute(&self, pos: BlockPos) -> f64 {
        // TODO: actually cache something here
        self.0.compute(pos)
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        self.0.fill(slice, context_provider)
    }

    fn min(&self) -> f64 {
        self.0.min()
    }

    fn max(&self) -> f64 {
        self.0.max()
    }
}
