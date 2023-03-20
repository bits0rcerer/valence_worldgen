use valence_protocol::block_pos::BlockPos;

use crate::density_function::DensityFunction;

pub struct RangeChoice {
    input: Box<dyn DensityFunction>,
    min_inclusive: f64,
    max_exclusive: f64,
    when_in_range: Box<dyn DensityFunction>,
    when_out_of_range: Box<dyn DensityFunction>,
}

impl RangeChoice {
    pub fn new(
        input: Box<dyn DensityFunction>,
        min_inclusive: f64,
        max_exclusive: f64,
        when_in_range: Box<dyn DensityFunction>,
        when_out_of_range: Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        Box::new(Self {
            input,
            min_inclusive,
            max_exclusive,
            when_in_range,
            when_out_of_range,
        })
    }
}

impl DensityFunction for RangeChoice {
    fn compute(&self, pos: BlockPos) -> f64 {
        let choice = self.input.compute(pos);

        if choice >= self.min_inclusive && choice < self.max_exclusive {
            self.when_in_range.compute(pos)
        } else {
            self.when_out_of_range.compute(pos)
        }
    }

    fn map(
        &self,
        visitor: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        RangeChoice::new(
            self.input.map(visitor),
            self.min_inclusive,
            self.max_exclusive,
            self.when_in_range.map(visitor),
            self.when_out_of_range.map(visitor),
        )
    }

    fn min(&self) -> f64 {
        f64::min(self.when_in_range.min(), self.when_out_of_range.min())
    }

    fn max(&self) -> f64 {
        f64::max(self.when_in_range.max(), self.when_out_of_range.max())
    }
}
