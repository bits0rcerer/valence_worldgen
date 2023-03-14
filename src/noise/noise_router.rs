use crate::density_function::deserialize::DensityFunctionTree;

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