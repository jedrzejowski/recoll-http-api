use serde::de::DeserializeOwned;
use anyhow::Result;

pub const ENV_PREFIX: &str = "FINDEX";

pub fn get_prefixed_env<K: AsRef<str>>(name: K) -> Option<String> {
  std::env::var(format!("{}_API_KEY", name.as_ref())).ok()
}

pub fn read_env_config<T: DeserializeOwned>(name: &str) -> Result<T> {
  return envy::prefixed(format!("{}_{}", ENV_PREFIX, name)).from_env::<T>()
    .map_err(anyhow::Error::msg);
}