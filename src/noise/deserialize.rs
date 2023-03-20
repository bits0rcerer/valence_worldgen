use serde::Deserialize;
use valence_protocol::block::BlockState;

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
    pub noise_settings: NoiseSettings,
    pub default_block: BlockState,
    pub default_fluid: BlockState,
    pub noise_router: NoiseRouterBlueprint,
    pub spawn_target: Vec<ClimatePoint>,
    pub sea_level: i32,
    pub disable_mob_generation: bool,
    pub aquifers_enabled: bool,
    pub ore_veins_enabled: bool,
    #[serde(rename = "legacy_random_source")]
    pub random_source_kind: random::Kind,

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
    pub barrier: DensityFunctionTree,
    pub continents: DensityFunctionTree,
    pub depth: DensityFunctionTree,
    pub erosion: DensityFunctionTree,
    pub final_density: DensityFunctionTree,
    pub fluid_level_floodedness: DensityFunctionTree,
    pub fluid_level_spread: DensityFunctionTree,
    pub initial_density_without_jaggedness: DensityFunctionTree,
    pub lava: DensityFunctionTree,
    pub ridges: DensityFunctionTree,
    pub temperature: DensityFunctionTree,
    pub vegetation: DensityFunctionTree,
    pub vein_gap: DensityFunctionTree,
    pub vein_ridged: DensityFunctionTree,
    pub vein_toggle: DensityFunctionTree,
}
