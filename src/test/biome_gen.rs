use std::sync::Arc;

use valence::prelude::ident;

use crate::random::random_state::RandomState;
use crate::registry::mc_meta::McMetaRegistry;
use crate::registry::Registry;

#[test]
fn generate_biome() {
    let registry = Arc::new(McMetaRegistry::new("./mcmeta", None));

    let settings = registry.noise_generator_settings(&ident!("minecraft:overworld"))
        .expect("should load overworld noise generator settings");

    let random_state = RandomState::new(&settings, registry.clone(), 6646468147532173577);
    let temperature = settings.noise_router.temperature.compile(&random_state).expect("density function should compile");
    let humidity = settings.noise_router.vegetation.compile(&random_state).expect("density function should compile");
    let continentalness = settings.noise_router.continents.compile(&random_state).expect("density function should compile");
    let erosion = settings.noise_router.erosion.compile(&random_state).expect("density function should compile");
    let depth = settings.noise_router.depth.compile(&random_state).expect("density function should compile");
    let weirdness = settings.noise_router.ridges.compile(&random_state).expect("density function should compile");

    let pos = valence::prelude::BlockPos::new(2048, 64, 2048);

    fn quantized(f: f64) -> i64 {
        (f * 10000_f64) as i64
    }

    assert_eq!(quantized(temperature.compute(pos)), -4380);
    assert_eq!(quantized(humidity.compute(pos)), 1784);
    assert_eq!(quantized(continentalness.compute(pos)), -2851);
    assert_eq!(quantized(erosion.compute(pos)), 3537);
    assert_eq!(quantized(depth.compute(pos)), -1237);
    assert_eq!(quantized(weirdness.compute(pos)), 2688);
}