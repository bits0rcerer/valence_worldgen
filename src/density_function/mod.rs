use valence::prelude::BlockPos;

pub(crate) mod deserialize;

#[cfg(test)]
mod test;

mod abs;
mod constant;
mod cube;
mod half_negative;
mod quarter_negative;
mod square;
mod squeeze;
mod transformer;
mod commutative;
mod max;
mod min;
mod add;
mod mul;

pub trait DensityFunction {
    fn compute(&self, pos: BlockPos) -> f64;
    fn map(
        &self,
        visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction>;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
}

fn sort_min_max(min: f64, max: f64) -> (f64, f64) {
    match (min, max) {
        (min, max) if min < max => (min, max),
        (min, max) if min >= max => (max, min),
        _ => unreachable!(),
    }
}
