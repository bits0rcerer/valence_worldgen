use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use serde::de;
use serde::de::Error;
use valence::prelude::Ident;

use crate::density_function::deserialize::DensityFunctionTree;
use crate::noise::deserialize::{NoiseGeneratorSettings, NoiseParameters};
use crate::registry::Registry;

type Cache<T> = RwLock<HashMap<Ident<String>, Arc<T>>>;

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

    fn load_from_file<T>(&self, path: &PathBuf) -> serde_json::Result<T>
        where
            T: de::DeserializeOwned,
    {
        let json_file_path = self.mcmeta_root.join(path);

        let f = match File::open(json_file_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                return Err(serde_json::Error::custom(format!(
                    "unable to open {}, Error: {}",
                    json_file_path.display(),
                    e
                )));
            }
        };

        serde_json::from_reader::<File, T>(f)
    }

    fn cached<T, H: FnMut(&Ident<String>, T) -> serde_json::Result<T>>(
        &self,
        id: &Ident<String>,
        map: &Cache<T>,
        path: &PathBuf,
        mut hydration_visitor: H,
    ) -> serde_json::Result<Arc<T>>
        where
            T: de::DeserializeOwned,
    {
        match map.read() {
            Ok(map) => {
                if map.contains_key(id) {
                    return Ok(map.get(id).unwrap().clone());
                }
            }
            Err(e) => {
                return Err(Error::custom(format!(
                    "unable to acquire lock on cache map, {}",
                    e
                )));
            }
        }

        return match self.load_from_file(path) {
            Ok(object) => match map.write() {
                Ok(mut map) => {
                    if map.contains_key(id) {
                        return Ok(map.get(id).unwrap().clone());
                    }

                    let arc = Arc::from(match hydration_visitor(id, object) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    });
                    map.insert(id.clone(), arc.clone());
                    Ok(arc)
                }
                Err(e) => Err(Error::custom(format!(
                    "unable to acquire lock on map, {}",
                    e
                ))),
            },
            Err(e) => Err(e),
        };
    }

    fn data_path<'a>(path: &'a str, tag: &'a Ident<String>) -> PathBuf {
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

    fn density_function(&self, id: Ident<String>) -> eyre::Result<Arc<DensityFunctionTree>> {
        Ok(self.cached(
            &id,
            &self.density_function_cache,
            &McMetaRegistry::data_path("worldgen/density_function", &id),
            |_, tree| Ok(tree),
        )?)
    }

    fn noise(&self, id: Ident<String>) -> eyre::Result<Arc<NoiseParameters>> {
        Ok(self.cached(
            &id,
            &self.noise_cache,
            &McMetaRegistry::data_path("worldgen/noise", &id),
            |_, tree| Ok(tree),
        )?)
    }

    fn noise_generator_settings(&self, id: Ident<String>) -> eyre::Result<Arc<NoiseGeneratorSettings>> {
        Ok(self.cached(
            &id,
            &self.noise_generator_settings_cache,
            &McMetaRegistry::data_path("worldgen/noise_settings", &id),
            |_, tree| Ok(tree),
        )?)
    }
}
