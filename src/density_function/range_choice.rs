use valence_protocol::block_pos::BlockPos;

use crate::density_function::{ContextProvider, DensityFunction};

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

    fn fill(&self, slice: &mut [f64], context_provider: &dyn ContextProvider) {
        self.input.fill(slice, context_provider);

        slice.iter_mut().enumerate()
            .for_each(|(i, v)| {
                if *v >= self.min_inclusive && *v < self.max_exclusive {
                    *v = self.when_in_range.compute(context_provider.for_index(i))
                } else {
                    *v = self.when_out_of_range.compute(context_provider.for_index(i))
                }
            })
    }

    fn min(&self) -> f64 {
        f64::min(self.when_in_range.min(), self.when_out_of_range.min())
    }

    fn max(&self) -> f64 {
        f64::max(self.when_in_range.max(), self.when_out_of_range.max())
    }
}
