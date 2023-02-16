use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize};

pub const ENV_PREFIX: &str = "FINDEX";

pub fn get_prefixed_env<K: AsRef<str>>(name: K) -> Option<String> {
  std::env::var(format!("{}_{}", ENV_PREFIX, name.as_ref())).ok()
}

pub fn read_env_config<T: DeserializeOwned>(name: &str) -> Result<T> {
  return envy::prefixed(format!("{}_{}_", ENV_PREFIX, name))
    .from_env::<T>()
    .map_err(anyhow::Error::msg);
}

#[derive(Debug, Deserialize)]
pub struct BinPaths {
  #[serde(default = "firejail_bin_default")]
  pub firejail: String,
  #[serde(default = "recollq_bin_default")]
  pub recollq: String,
  #[serde(default = "recollindex_bin_default")]
  pub recollindex: String,
}

impl BinPaths {
  pub fn get() -> &'static BinPaths {
    static INSTANCE: OnceCell<BinPaths> = OnceCell::new();
    &INSTANCE.get_or_init(|| read_env_config("BIN").unwrap())
  }

  pub fn firejail() -> &'static str {
    Self::get().firejail.as_str()
  }

  pub fn recollq() -> &'static str {
    Self::get().recollq.as_str()
  }

  pub fn recollindex() -> &'static str {
    Self::get().recollindex.as_str()
  }
}

fn firejail_bin_default() -> String {
  return "/usr/bin/firejail".to_string();
}

fn recollq_bin_default() -> String {
  return "/usr/bin/recollq".to_string();
}

fn recollindex_bin_default() -> String {
  return "/usr/bin/recollindex".to_string();
}
