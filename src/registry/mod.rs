use std::sync::Arc;

use valence::prelude::Ident;

use crate::density_function::DensityFunction;
use crate::density_function::deserialize::DensityFunctionTree;
use crate::noise::NoiseParameters;

pub mod mc_meta;

pub trait Registry {
    fn root_registry(&self) -> &dyn Registry;
    fn density_function(
        &self,
        id: Ident<String>,
        seed: u64,
    ) -> eyre::Result<Arc<DensityFunctionTree>>;
    fn noise(&self,
             id: Ident<String>,
             seed: u64,
    ) -> eyre::Result<Arc<NoiseParameters>>;
}
