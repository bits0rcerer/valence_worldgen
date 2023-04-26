use valence_core::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};
use crate::spline::{Built, CubicSpline};

impl DensityFunction for CubicSpline<Built> {
    fn compute(&self, pos: BlockPos) -> f64 {
        self.compute(pos) as f64
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        context_provider.fill_direct(slice, self)
    }

    fn min(&self) -> f64 {
        self.min() as f64
    }

    fn max(&self) -> f64 {
        self.max() as f64
    }
}
