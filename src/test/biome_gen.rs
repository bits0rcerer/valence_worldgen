use std::sync::Arc;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use serde::Deserialize;
use valence::prelude::ident;

use crate::random::random_state::RandomState;
use crate::registry::mc_meta::McMetaRegistry;
use crate::registry::Registry;

#[test]
fn generate_biomes() {
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

    fn quantized(f: f64) -> i32 {
        (f * 10000_f64) as i32
    }

    #[derive(Deserialize)]
    struct Sample {
        x: i32,
        y: i32,
        z: i32,
        temperature: i32,
        humidity: i32,
        continentalness: i32,
        erosion: i32,
        depth: i32,
        weirdness: i32,
    }

    let samples = csv::Reader::from_path("src/test/biome_parameters_sample.csv")
        .expect("should load samples")
        .deserialize::<Sample>()
        .map(|o| o.expect("should be a valid sample"))
        .collect::<Vec<_>>();

    let mut do_test = |value: i32, sample: i32| {
        let diff = (sample - value).abs();
        assert!(diff <= 1);
        diff
    };

    let small_errors: i32 = samples.par_iter().map(|s| {
        let pos = valence::prelude::BlockPos::new(s.x, s.y, s.z);

        do_test(quantized(temperature.compute(pos)), s.temperature)
            + do_test(quantized(humidity.compute(pos)), s.humidity)
            + do_test(quantized(continentalness.compute(pos)), s.continentalness)
            + do_test(quantized(erosion.compute(pos)), s.erosion)
            + do_test(quantized(depth.compute(pos)), s.depth)
            + do_test(quantized(weirdness.compute(pos)), s.weirdness)
    }).sum();

    println!("Tested {} samples, got {small_errors} small errors (quantized parameter - sample <= 1)", samples.len())
}
