use valence_protocol::block_pos::BlockPos;

use crate::density_function::DensityFunction;
use crate::spline::{Built, CubicSpline};

impl DensityFunction for CubicSpline<Built> {
    fn compute(&self, pos: BlockPos) -> f64 {
        self.compute(pos) as f64
    }

    fn map(
        &self,
        _: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        todo!()
    }

    fn min(&self) -> f64 {
        self.min() as f64
    }

    fn max(&self) -> f64 {
        self.max() as f64
    }
}
