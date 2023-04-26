use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use eyre::eyre;
use serde::de;
use valence_core::ident::Ident;

use crate::density_function::deserialize::DensityFunctionTree;
use crate::noise::deserialize::{NoiseGeneratorSettings, NoiseParameters};
use crate::registry::Registry;

type Cache<T> = RwLock<HashMap<String, Arc<T>>>;

pub struct McMetaRegistry {
    root_registry: Option<Box<dyn Registry>>,
    mcmeta_root: PathBuf,

    density_function_cache: Cache<DensityFunctionTree>,
    noise_cache: Cache<NoiseParameters>,
    noise_generator_settings_cache: Cache<NoiseGeneratorSettings>,
}

impl McMetaRegistry {
    pub fn new<S: AsRef<str>>(root_path: S, root_registry: Option<Box<dyn Registry>>) -> Self {
        Self {
            mcmeta_root: root_path.as_ref().to_string().into(),
            root_registry,
            ..Default::default()
        }
    }

    fn load_from_file<T>(&self, path: &PathBuf) -> eyre::Result<T>
    where
        T: de::DeserializeOwned,
    {
        let json_file_path = self.mcmeta_root.join(path);

        let f = match File::open(json_file_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                return Err(eyre!(
                    "unable to open {}, Error: {}",
                    json_file_path.display(),
                    e
                ));
            }
        };

        let deserializer = &mut serde_json::Deserializer::from_reader(f);
        serde_path_to_error::deserialize(deserializer)
            .map_err(|e| eyre!("unable to deserialize {path:?}::{} - {e}", e.path()))
    }

    fn cached<T, H: FnMut(&Ident<&str>, T) -> eyre::Result<T>>(
        &self,
        id: &Ident<&str>,
        map: &Cache<T>,
        path: &PathBuf,
        mut hydration_visitor: H,
    ) -> eyre::Result<Arc<T>>
    where
        T: de::DeserializeOwned,
    {
        match map.read() {
            Ok(map) => {
                if map.contains_key(id.as_str()) {
                    return Ok(map.get(id.as_str()).unwrap().clone());
                }
            }
            Err(e) => {
                return Err(eyre!("unable to acquire lock on cache map, {}", e));
            }
        }

        return match self.load_from_file(path) {
            Ok(object) => match map.write() {
                Ok(mut map) => {
                    if map.contains_key(id.as_str()) {
                        return Ok(map.get(id.as_str()).unwrap().clone());
                    }

                    let arc = Arc::from(match hydration_visitor(id, object) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    });
                    map.insert(id.to_string(), arc.clone());
                    Ok(arc)
                }
                Err(e) => Err(eyre!("unable to acquire lock on map, {}", e)),
            },
            Err(e) => Err(e),
        };
    }

    fn data_path<'a>(path: &'a str, tag: &'a Ident<&str>) -> PathBuf {
        format!("data/{}/{}/{}.json", tag.namespace(), path, tag.path()).into()
    }
}

impl Default for McMetaRegistry {
    fn default() -> Self {
        McMetaRegistry {
            root_registry: None,
            mcmeta_root: PathBuf::from_str("./mcmeta").unwrap(),
            density_function_cache: Default::default(),
            noise_cache: Default::default(),
            noise_generator_settings_cache: Default::default(),
        }
    }
}

impl Registry for McMetaRegistry {
    fn root_registry(&self) -> &dyn Registry {
        match &self.root_registry {
            None => self,
            Some(r) => r.as_ref(),
        }
    }

    fn density_function(&self, id: &Ident<&str>) -> eyre::Result<Arc<DensityFunctionTree>> {
        self.cached(
            id,
            &self.density_function_cache,
            &McMetaRegistry::data_path("worldgen/density_function", id),
            |_, tree| Ok(tree),
        )
    }

    fn noise(&self, id: &Ident<&str>) -> eyre::Result<Arc<NoiseParameters>> {
        self.cached(
            id,
            &self.noise_cache,
            &McMetaRegistry::data_path("worldgen/noise", id),
            |_, tree| Ok(tree),
        )
    }

    fn noise_generator_settings(
        &self,
        id: &Ident<&str>,
    ) -> eyre::Result<Arc<NoiseGeneratorSettings>> {
        self.cached(
            id,
            &self.noise_generator_settings_cache,
            &McMetaRegistry::data_path("worldgen/noise_settings", id),
            |_, tree| Ok(tree),
        )
    }
}
