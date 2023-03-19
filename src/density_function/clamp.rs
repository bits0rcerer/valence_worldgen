use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

pub struct Clamp {
    f: Box<dyn DensityFunction>,
    min: f64,
    max: f64,
}

impl Clamp {
    pub fn new(f: Box<dyn DensityFunction>, min: f64, max: f64) -> Box<dyn DensityFunction> {
        Box::new(
            Clamp {
                f,
                min,
                max,
            }
        )
    }
}

impl DensityFunction for Clamp {
    fn compute(&self, pos: BlockPos) -> f64 {
        dbg!(f64::clamp(self.f.compute(pos), self.min, self.max))
    }

    fn map(&self, visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Clamp::new(self.f.map(visitor), self.min, self.max)
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}