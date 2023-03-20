use valence_protocol::block_pos::BlockPos;

#[cfg(test)]
mod test;

mod abs;
mod add;
mod cache_2d;
mod cache_once;
mod clamp;
mod commutative;
pub mod compile;
mod constant;
mod cube;
pub mod deserialize;
mod flat_cache;
mod half_negative;
mod max;
mod min;
mod mul;
mod noise;
mod quarter_negative;
mod range_choice;
mod spline;
mod square;
mod squeeze;
mod transformer;
pub(crate) mod y_clamped_gradient;

pub trait DensityFunction: Send + Sync {
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
