use std::sync::Arc;

use valence::prelude::Ident;

use crate::density_function::deserialize::DensityFunctionTree;
use crate::noise::deserialize::{NoiseGeneratorSettings, NoiseParameters};

pub mod mc_meta;

pub trait Registry {
    fn root_registry(&self) -> &dyn Registry;
    fn density_function(&self, id: &Ident<String>) -> eyre::Result<Arc<DensityFunctionTree>>;
    fn noise(&self, id: &Ident<String>) -> eyre::Result<Arc<NoiseParameters>>;
    fn noise_generator_settings(&self, id: &Ident<String>) -> eyre::Result<Arc<NoiseGeneratorSettings>>;
}
