use serde::de::DeserializeOwned;

pub const ENV_PREFIX: &str = "WINDEX";

pub fn get_prefixed_env<K: AsRef<str>>(name: K) -> Option<String> {
  std::env::var(format!("{}_API_KEY", name.as_ref())).ok()
}

pub fn read_env_config<T: DeserializeOwned>(name: &str) -> T {
  return envy::prefixed(format!("{}_{}", ENV_PREFIX, name)).from_env::<T>().unwrap();
}