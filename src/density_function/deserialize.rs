use std::rc::Rc;

use serde::Deserialize;
use valence::prelude::Ident;

use crate::density_function::abs::abs;
use crate::density_function::add::add;
use crate::density_function::clamp::Clamp;
use crate::density_function::constant::Constant;
use crate::density_function::cube::cube;
use crate::density_function::DensityFunction;
use crate::density_function::half_negative::half_negative;
use crate::density_function::max::max;
use crate::density_function::min::min;
use crate::density_function::mul::mul;
use crate::density_function::quarter_negative::quarter_negative;
use crate::density_function::square::square;
use crate::density_function::squeeze::squeeze;
use crate::registry::Registry;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum DensityFunctionTree {
    Constant(f64),
    Reference(Ident<String>),
    Inline(InlineDensityFunctionTree),
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(crate) enum InlineDensityFunctionTree {
    #[serde(rename = "minecraft:abs")]
    Abs { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:add")]
    Add {
        argument1: Rc<DensityFunctionTree>,
        argument2: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:blend_density")]
    BlendDensity { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_2d")]
    Cache2D { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_all_in_cell")]
    CacheAllInCell { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_once")]
    CacheOnce { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:clamp")]
    Clamp {
        input: Rc<DensityFunctionTree>,
        min: f64,
        max: f64,
    },

    #[serde(rename = "minecraft:constant")]
    Constant { argument: f64 },

    #[serde(rename = "minecraft:cube")]
    Cube { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:flat_cache")]
    FlatCache { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:half_negative")]
    HalfNegative { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:interpolated")]
    Interpolated { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:max")]
    Max {
        argument1: Rc<DensityFunctionTree>,
        argument2: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:min")]
    Min {
        argument1: Rc<DensityFunctionTree>,
        argument2: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:mul")]
    Mul {
        argument1: Rc<DensityFunctionTree>,
        argument2: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:noise")]
    Noise {
        noise: Ident<String>,
        xz_scale: f64,
        y_scale: f64,
    },

    #[serde(rename = "minecraft:old_blended_noise")]
    OldBlendNoise {
        xz_scale: f64,
        y_scale: f64,
        xz_factor: f64,
        y_factor: f64,
        smear_scale_multiplier: u8,
    },

    #[serde(rename = "minecraft:quarter_negative")]
    QuarterNegative { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:range_choice")]
    RangeChoice {
        input: Rc<DensityFunctionTree>,
        min_inclusive: f64,
        max_exclusive: f64,
        when_in_range: Rc<DensityFunctionTree>,
        when_out_of_range: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:shift")]
    Shift {
        #[serde(rename = "argument")]
        noise: Ident<String>,
    },

    #[serde(rename = "minecraft:shift_a")]
    ShiftA {
        #[serde(rename = "argument")]
        noise: Ident<String>,
    },

    #[serde(rename = "minecraft:shift_b")]
    ShiftB {
        #[serde(rename = "argument")]
        noise: Ident<String>,
    },

    #[serde(rename = "minecraft:shifted_noise")]
    ShiftedNoise {
        #[serde(rename = "input")]
        noise: Ident<String>,
        xz_scale: f64,
        y_scale: f64,
        shift_x: Rc<DensityFunctionTree>,
        shift_y: Rc<DensityFunctionTree>,
        shift_z: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:slide")]
    Slide { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:spline")]
    Spline { spline: CubicSpline },

    #[serde(rename = "minecraft:square")]
    Square { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:squeeze")]
    Squeeze { argument: Rc<DensityFunctionTree> },

    #[serde(rename = "minecraft:weird_scaled_sampler")]
    WeirdScaledSampler {
        rarity_value_mapper: RarityValueMapper,
        noise: Ident<String>,
        input: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:y_clamped_gradient")]
    YClampedGradient {
        from_y: i32,
        to_y: i32,
        from_value: f64,
        to_value: f64,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum Spline {
    Constant(f64),
    CubicSpline {
        #[serde(flatten)]
        cubic_spline: CubicSpline,
    },
}

#[derive(Deserialize)]
pub(crate) struct CubicSpline {
    coordinate: Rc<DensityFunctionTree>,
    points: Vec<CubicSplinePoint>,
}

#[derive(Deserialize)]
pub(crate) enum CubicSplinePoint {
    Constant {
        location: f64,
        derivative: f64,
        value: f64,
    },
    Variable {
        location: f64,
        derivative: f64,
        value: Vec<CubicSpline>,
    },
}

#[derive(Deserialize)]
pub(crate) enum RarityValueMapper {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
}

impl DensityFunctionTree {
    pub fn compile(&self, seed: u64, r: &dyn Registry) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            DensityFunctionTree::Constant(arg) => Ok(Constant::new(*arg)),
            DensityFunctionTree::Reference(id) => r.density_function(id.clone(), seed),
            DensityFunctionTree::Inline(f) => f.compile(seed, r),
        }
    }
}

impl InlineDensityFunctionTree {
    pub fn compile(&self, seed: u64, r: &dyn Registry) -> eyre::Result<Box<dyn DensityFunction>> {
        match self {
            InlineDensityFunctionTree::Abs { argument } => Ok(abs(argument.compile(seed, r)?)),
            InlineDensityFunctionTree::Square { argument } => Ok(square(argument.compile(seed, r)?)),
            InlineDensityFunctionTree::Cube { argument } => Ok(cube(argument.compile(seed, r)?)),
            InlineDensityFunctionTree::HalfNegative { argument } => Ok(half_negative(argument.compile(seed, r)?)),
            InlineDensityFunctionTree::QuarterNegative { argument } => Ok(quarter_negative(argument.compile(seed, r)?)),
            InlineDensityFunctionTree::Squeeze { argument } => Ok(squeeze(argument.compile(seed, r)?)),

            InlineDensityFunctionTree::Max { argument1, argument2 } => Ok(max(argument1.compile(seed, r)?, argument2.compile(seed, r)?)),
            InlineDensityFunctionTree::Min { argument1, argument2 } => Ok(min(argument1.compile(seed, r)?, argument2.compile(seed, r)?)),
            InlineDensityFunctionTree::Add { argument1, argument2 } => Ok(add(argument1.compile(seed, r)?, argument2.compile(seed, r)?)),
            InlineDensityFunctionTree::Mul { argument1, argument2 } => Ok(mul(argument1.compile(seed, r)?, argument2.compile(seed, r)?)),

            InlineDensityFunctionTree::Clamp { input, min, max } => Ok(Clamp::new(input.compile(seed, r)?, *min, *max)),
            // TODO: InlineDensityFunctionTree::BlendDensity { .. } => {}
            // TODO: InlineDensityFunctionTree::Cache2D { .. } => {}
            // TODO: InlineDensityFunctionTree::CacheAllInCell { .. } => {}
            // TODO: InlineDensityFunctionTree::CacheOnce { .. } => {}
            // TODO: InlineDensityFunctionTree::Constant { .. } => {}
            // TODO: InlineDensityFunctionTree::FlatCache { .. } => {}
            // TODO: InlineDensityFunctionTree::Interpolated { .. } => {}
            // TODO: InlineDensityFunctionTree::Noise { .. } => {}
            // TODO: InlineDensityFunctionTree::OldBlendNoise { .. } => {}
            // TODO: InlineDensityFunctionTree::RangeChoice { .. } => {}
            // TODO: InlineDensityFunctionTree::Shift { .. } => {}
            // TODO: InlineDensityFunctionTree::ShiftA { .. } => {}
            // TODO: InlineDensityFunctionTree::ShiftB { .. } => {}
            // TODO: InlineDensityFunctionTree::ShiftedNoise { .. } => {}
            // TODO: InlineDensityFunctionTree::Slide { .. } => {}
            // TODO: InlineDensityFunctionTree::Spline { .. } => {}
            // TODO: InlineDensityFunctionTree::WeirdScaledSampler { .. } => {}
            // TODO: InlineDensityFunctionTree::YClampedGradient { .. } => {}
            _ => todo!(),
        }
    }
}
