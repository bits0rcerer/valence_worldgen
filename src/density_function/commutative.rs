use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;

#[derive(Copy, Clone)]
pub enum Operation {
    Add,
    Multiply,
    Min,
    Max,
}

impl Operation {
    fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Min => f64::min(a, b),
            Operation::Max => f64::max(a, b),
        }
    }

    fn min(&self, a: &dyn DensityFunction, b: &dyn DensityFunction) -> f64 {
        match self {
            Operation::Add => a.min() + b.min(),
            Operation::Min => f64::min(a.min(), b.min()),
            Operation::Max => f64::max(a.min(), b.min()),
            Operation::Multiply =>
                if a.min() > 0.0 && b.min() > 0.0 {
                    a.min() * b.min()
                } else if a.max() < 0.0 && b.max() < 0.0 {
                    a.max() * b.max()
                } else {
                    f64::min(a.min() * b.max(), a.max() * b.min())
                }
        }
    }

    fn max(&self, a: &dyn DensityFunction, b: &dyn DensityFunction) -> f64 {
        match self {
            Operation::Add => a.max() + b.max(),
            Operation::Min => f64::min(a.max(), b.max()),
            Operation::Max => f64::max(a.max(), b.max()),
            Operation::Multiply =>
                if a.min() > 0.0 && b.min() > 0.0 {
                    a.max() * b.max()
                } else if a.max() < 0.0 && b.max() < 0.0 {
                    a.min() * b.min()
                } else {
                    f64::max(a.min() * b.min(), a.max() * b.max())
                }
        }
    }
}

pub struct Commutative {
    f1: Box<dyn DensityFunction>,
    f2: Box<dyn DensityFunction>,
    operation: Operation,
    min: f64,
    max: f64,
}

impl Commutative {
    pub fn new(f1: Box<dyn DensityFunction>, f2: Box<dyn DensityFunction>, operation: Operation) -> Box<dyn DensityFunction> {
        Box::new(
            Self {
                min: operation.min(f1.as_ref(), f2.as_ref()),
                max: operation.max(f1.as_ref(), f2.as_ref()),
                f1,
                f2,
                operation,
            }
        )
    }
}

impl DensityFunction for Commutative {
    fn compute(&self, pos: BlockPos) -> f64 {
        self.operation.apply(self.f1.compute(pos), self.f2.compute(pos))
    }

    fn map(&self, visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        visitor(Commutative::new(self.f1.map(visitor), self.f2.map(visitor), self.operation).as_ref())
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}