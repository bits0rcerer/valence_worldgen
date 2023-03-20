use valence_protocol::block_pos::BlockPos;

use crate::density_function::DensityFunction;

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

    fn map(
        &self,
        visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        visitor(self)
    }

    fn min(&self) -> f64 {
        self.0
    }

    fn max(&self) -> f64 {
        self.0
    }
}
