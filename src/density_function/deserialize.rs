use std::rc::Rc;

use serde::Deserialize;
use valence::prelude::Ident;

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
    Abs {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:add")]
    Add {
        argument1: Rc<DensityFunctionTree>,
        argument2: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:blend_density")]
    BlendDensity {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:cache_2d")]
    Cache2D {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:cache_all_in_cell")]
    CacheAllInCell {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:cache_once")]
    CacheOnce {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:clamp")]
    Clamp {
        input: Rc<DensityFunctionTree>,
        min: f64,
        max: f64,
    },

    #[serde(rename = "minecraft:constant")]
    Constant {
        argument: f64
    },

    #[serde(rename = "minecraft:cube")]
    Cube {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:flat_cache")]
    FlatCache {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:half_negative")]
    HalfNegative {
        argument: Rc<DensityFunctionTree>
    },

    #[serde(rename = "minecraft:interpolated")]
    Interpolated {
        argument: Rc<DensityFunctionTree>
    },

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
    QuarterNegative {
        argument: Rc<DensityFunctionTree>,
    },

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
    Slide {
        argument: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:spline")]
    Spline {
        spline: CubicSpline,
    },

    #[serde(rename = "minecraft:square")]
    Square {
        argument: Rc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:squeeze")]
    Squeeze {
        argument: Rc<DensityFunctionTree>,
    },

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