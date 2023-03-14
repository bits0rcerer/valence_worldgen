use std::simd::f64x4;

use crate::density_function;
use crate::density_function::abs::abs;
use crate::density_function::add::add;
use crate::density_function::clamp::Clamp;
use crate::density_function::constant::Constant;
use crate::density_function::cube::cube;
use crate::density_function::DensityFunction;
use crate::density_function::deserialize::{DensityFunctionTree, InlineDensityFunctionTree};
use crate::density_function::half_negative::half_negative;
use crate::density_function::max::max;
use crate::density_function::min::min;
use crate::density_function::mul::mul;
use crate::density_function::quarter_negative::quarter_negative;
use crate::density_function::range_choice::RangeChoice;
use crate::density_function::square::square;
use crate::density_function::squeeze::squeeze;
use crate::random::random_state::RandomState;
use crate::registry::Registry;

impl DensityFunctionTree {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            DensityFunctionTree::Constant(arg) => Ok(Constant::new(*arg)),
            DensityFunctionTree::Reference(id) => random_state.registry.density_function(id)?.compile(random_state),
            DensityFunctionTree::Inline(f) => f.compile(random_state),
        }
    }
}

impl InlineDensityFunctionTree {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            InlineDensityFunctionTree::Constant { argument } => Ok(Constant::new(*argument)),

            InlineDensityFunctionTree::Abs { argument } => Ok(abs(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Square { argument } => Ok(square(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Cube { argument } => Ok(cube(argument.compile(random_state)?)),
            InlineDensityFunctionTree::HalfNegative { argument } => Ok(half_negative(argument.compile(random_state)?)),
            InlineDensityFunctionTree::QuarterNegative { argument } => Ok(quarter_negative(argument.compile(random_state)?)),
            InlineDensityFunctionTree::Squeeze { argument } => Ok(squeeze(argument.compile(random_state)?)),

            InlineDensityFunctionTree::Max { argument1, argument2 } => Ok(max(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Min { argument1, argument2 } => Ok(min(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Add { argument1, argument2 } => Ok(add(argument1.compile(random_state)?, argument2.compile(random_state)?)),
            InlineDensityFunctionTree::Mul { argument1, argument2 } => Ok(mul(argument1.compile(random_state)?, argument2.compile(random_state)?)),

            InlineDensityFunctionTree::Clamp { input, min, max } => Ok(Clamp::new(input.compile(random_state)?, *min, *max)),

            InlineDensityFunctionTree::Cache2D { argument } => todo!(),
            InlineDensityFunctionTree::CacheAllInCell { argument } => todo!(),
            InlineDensityFunctionTree::CacheOnce { argument } => todo!(),
            InlineDensityFunctionTree::FlatCache { argument } => todo!(),
            InlineDensityFunctionTree::Interpolated { argument } => todo!(),

            InlineDensityFunctionTree::Noise { noise, xz_scale, y_scale } => density_function::noise::noise(noise, random_state, 1.0, f64x4::from_array([*xz_scale, *y_scale, *xz_scale, 0.0])),
            InlineDensityFunctionTree::Shift { noise } => density_function::noise::noise(noise, random_state, 4.0, f64x4::from_array([0.25, 0.25, 0.25, 0.0])),
            InlineDensityFunctionTree::ShiftA { noise } => density_function::noise::noise(noise, random_state, 4.0, f64x4::from_array([0.25, 0.0, 0.25, 0.0])),
            InlineDensityFunctionTree::ShiftB { noise } => density_function::noise::noise(noise, random_state, 4.0, f64x4::from_array([0.25, 0.25, 0.0, 0.0])),
            InlineDensityFunctionTree::ShiftedNoise { noise, shift_x, shift_y, shift_z, xz_scale, y_scale } =>
                density_function::noise::shifted_noise(noise, random_state, 1.0, f64x4::from_array([*xz_scale, *y_scale, *xz_scale, 0.0]),
                                                       shift_x.compile(random_state)?, shift_y.compile(random_state)?, shift_z.compile(random_state)?,
                ),

            InlineDensityFunctionTree::RangeChoice { input, min_inclusive, max_exclusive, when_in_range, when_out_of_range } =>
                Ok(RangeChoice::new(input.compile(random_state)?, *min_inclusive, *max_exclusive, when_in_range.compile(random_state)?, when_out_of_range.compile(random_state)?)),
            InlineDensityFunctionTree::Spline { spline } => todo!(),
            InlineDensityFunctionTree::WeirdScaledSampler { noise, input, rarity_value_mapper } => todo!(),
            InlineDensityFunctionTree::YClampedGradient { from_y, to_y, from_value, to_value } => todo!(),

            // Blending
            InlineDensityFunctionTree::BlendDensity { argument } => todo!(),
            InlineDensityFunctionTree::OldBlendNoise { xz_scale, y_scale, xz_factor, y_factor, smear_scale_multiplier } => todo!(),

            #[deprecated]
            InlineDensityFunctionTree::Slide { argument } => todo!(),
        }
    }
}
