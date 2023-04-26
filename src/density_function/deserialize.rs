use std::sync::Arc;

use serde::Deserialize;
use valence_core::ident::Ident;

use crate::spline::{Blueprint, CubicSpline};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DensityFunctionTree {
    Constant(f64),
    Reference(String),
    Inline(InlineDensityFunctionTree),
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum InlineDensityFunctionTree {
    #[serde(rename = "minecraft:abs")]
    Abs { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:add")]
    Add {
        argument1: Arc<DensityFunctionTree>,
        argument2: Arc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:blend_density")]
    BlendDensity { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_2d")]
    Cache2D { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_all_in_cell")]
    CacheAllInCell { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:cache_once")]
    CacheOnce { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:flat_cache")]
    FlatCache { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:clamp")]
    Clamp {
        input: Arc<DensityFunctionTree>,
        min: f64,
        max: f64,
    },

    #[serde(rename = "minecraft:constant")]
    Constant { argument: f64 },

    #[serde(rename = "minecraft:cube")]
    Cube { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:half_negative")]
    HalfNegative { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:interpolated")]
    Interpolated { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:max")]
    Max {
        argument1: Arc<DensityFunctionTree>,
        argument2: Arc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:min")]
    Min {
        argument1: Arc<DensityFunctionTree>,
        argument2: Arc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:mul")]
    Mul {
        argument1: Arc<DensityFunctionTree>,
        argument2: Arc<DensityFunctionTree>,
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
    QuarterNegative { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:range_choice")]
    RangeChoice {
        input: Arc<DensityFunctionTree>,
        min_inclusive: f64,
        max_exclusive: f64,
        when_in_range: Arc<DensityFunctionTree>,
        when_out_of_range: Arc<DensityFunctionTree>,
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
        noise: Ident<String>,
        xz_scale: f64,
        y_scale: f64,
        shift_x: Arc<DensityFunctionTree>,
        shift_y: Arc<DensityFunctionTree>,
        shift_z: Arc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:slide")]
    Slide { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:spline")]
    Spline { spline: CubicSpline<Blueprint> },

    #[serde(rename = "minecraft:square")]
    Square { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:squeeze")]
    Squeeze { argument: Arc<DensityFunctionTree> },

    #[serde(rename = "minecraft:weird_scaled_sampler")]
    WeirdScaledSampler {
        rarity_value_mapper: RarityValueMapper,
        noise: Ident<String>,
        input: Arc<DensityFunctionTree>,
    },

    #[serde(rename = "minecraft:y_clamped_gradient")]
    YClampedGradient {
        from_y: i32,
        to_y: i32,
        from_value: f64,
        to_value: f64,
    },

    #[serde(rename = "minecraft:blend_offset")]
    BlendOffset {},

    #[serde(rename = "minecraft:blend_alpha")]
    BlendAlpha {},
}

#[derive(Deserialize)]
pub enum RarityValueMapper {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
}
