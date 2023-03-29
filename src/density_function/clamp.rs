use valence_protocol::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};

pub struct Clamp {
    f: Box<dyn DensityFunction>,
    min: f64,
    max: f64,
}

impl Clamp {
    pub fn new(f: Box<dyn DensityFunction>, min: f64, max: f64) -> Box<dyn DensityFunction> {
        Box::new(Clamp { f, min, max })
    }
}

impl DensityFunction for Clamp {
    fn compute(&self, pos: BlockPos) -> f64 {
        f64::clamp(self.f.compute(pos), self.min, self.max)
    }

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        self.f.fill(slice, context_provider);
        slice.iter_mut().for_each(|v| *v = v.clamp(self.min, self.max))
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}
