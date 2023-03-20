use std::sync::Arc;

use crate::noise::deserialize::NoiseGeneratorSettings;
use crate::random::PositionalRandomFactory;
use crate::registry::Registry;
use crate::surface::SurfaceSystem;

pub struct RandomState {
    pub(crate) random: Box<dyn PositionalRandomFactory>,
    pub(crate) seed: i64,
    pub(crate) registry: Arc<dyn Registry>,
    pub(crate) aquifer_random: Box<dyn PositionalRandomFactory>,
    pub(crate) ore_random: Box<dyn PositionalRandomFactory>,
    //pub(crate) noise_instance_cache: Map<Ident<String>, NormalNoise>                         // TODO: are cached noise instances useful? - Probably yes
    //pub(crate) random_factories_cache: Map<Ident<String>, Box<dyn PositionalRandomFactory>>  // TODO: are cached random factories useful? - I am not sure
    pub(crate) surface_system: SurfaceSystem,
}

impl RandomState {
    pub fn new(settings: &NoiseGeneratorSettings, registry: Arc<dyn Registry>, seed: i64) -> Self {
        let random = settings
            .random_source_kind
            .new_instance(seed)
            .fork_positional();

        Self {
            aquifer_random: random.with_hash_of("aquifer").fork_positional(),
            ore_random: random.with_hash_of("ore").fork_positional(),
            surface_system: SurfaceSystem,
            random,
            seed,
            registry,
        }
    }
}
