use valence::prelude::Ident;

use crate::density_function::DensityFunction;

pub mod mc_meta;

pub trait Registry {
    fn root_registry(&self) -> &dyn Registry;
    fn density_function(
        &self,
        id: Ident<String>,
        seed: u64,
    ) -> eyre::Result<Box<dyn DensityFunction>>;
}
