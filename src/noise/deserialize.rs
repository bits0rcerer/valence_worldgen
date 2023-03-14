use serde::Deserialize;
use valence::prelude::BlockState;

use crate::biome::ClimatePoint;
use crate::density_function::deserialize::DensityFunctionTree;
use crate::random;
use crate::surface::SurfaceRuleSource;

#[derive(Clone, Deserialize)]
pub struct NoiseParameters {
    #[serde(rename = "firstOctave")]
    pub first_octave: i32,
    pub amplitudes: Vec<f64>,
}

impl NoiseParameters {
    pub fn new(first_octave: i32, amplitudes: Vec<f64>) -> Self {
        Self {
            first_octave,
            amplitudes,
        }
    }
}

#[derive(Deserialize)]
pub struct NoiseGeneratorSettings {
    #[serde(rename = "noise")]
    pub(crate) noise_settings: NoiseSettings,
    pub(crate) default_block: BlockState,
    pub(crate) default_fluid: BlockState,
    pub(crate) noise_router: NoiseRouterBlueprint,
    pub(crate) spawn_target: Vec<ClimatePoint>,
    pub(crate) sea_level: i32,
    pub(crate) disable_mob_generation: bool,
    pub(crate) aquifers_enabled: bool,
    pub(crate) ore_veins_enabled: bool,
    pub(crate) random_source_kind: random::Kind,

    #[serde(skip)] // TODO: do not skip
    pub(crate) surface_rule: SurfaceRuleSource,
}

#[derive(Deserialize)]
pub struct NoiseSettings {
    min_y: i32,
    height: i32,
    #[serde(rename = "size_horizontal")]
    xz_size: i32,
    #[serde(rename = "size_vertical")]
    y_size: i32,
}

#[derive(Deserialize)]
pub struct NoiseRouterBlueprint {
    barrier: DensityFunctionTree,
    continents: DensityFunctionTree,
    depth: DensityFunctionTree,
    erosion: DensityFunctionTree,
    final_density: DensityFunctionTree,
    fluid_level_floodedness: DensityFunctionTree,
    fluid_level_spread: DensityFunctionTree,
    initial_density_without_jaggedness: DensityFunctionTree,
    lava: DensityFunctionTree,
    ridges: DensityFunctionTree,
    temperature: DensityFunctionTree,
    vegetation: DensityFunctionTree,
    vein_gap: DensityFunctionTree,
    vein_ridged: DensityFunctionTree,
    vein_toggle: DensityFunctionTree,
}
