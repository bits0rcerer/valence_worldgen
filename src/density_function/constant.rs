use valence_core::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};

pub struct Constant(f64);

impl Constant {
    pub(crate) fn new(arg: f64) -> Box<dyn DensityFunction> {
        Box::new(Constant(arg))
    }
}

impl DensityFunction for Constant {
    fn compute(&self, _: BlockPos) -> f64 {
        self.0
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        slice.iter_mut().for_each(|v| *v = self.0)
    }

    fn min(&self) -> f64 {
        self.0
    }

    fn max(&self) -> f64 {
        self.0
    }
}
