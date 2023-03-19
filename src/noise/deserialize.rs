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
    #[serde(rename = "legacy_random_source")]
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
    pub(crate) barrier: DensityFunctionTree,
    pub(crate) continents: DensityFunctionTree,
    pub(crate) depth: DensityFunctionTree,
    pub(crate) erosion: DensityFunctionTree,
    pub(crate) final_density: DensityFunctionTree,
    pub(crate) fluid_level_floodedness: DensityFunctionTree,
    pub(crate) fluid_level_spread: DensityFunctionTree,
    pub(crate) initial_density_without_jaggedness: DensityFunctionTree,
    pub(crate) lava: DensityFunctionTree,
    pub(crate) ridges: DensityFunctionTree,
    pub(crate) temperature: DensityFunctionTree,
    pub(crate) vegetation: DensityFunctionTree,
    pub(crate) vein_gap: DensityFunctionTree,
    pub(crate) vein_ridged: DensityFunctionTree,
    pub(crate) vein_toggle: DensityFunctionTree,
}
