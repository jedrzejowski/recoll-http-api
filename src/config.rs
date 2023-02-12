use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::file_index::FileIndex;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
  pub indexes: HashMap<String, FileIndex>
}

impl AppConfig {
  pub fn get_index<S: AsRef<str>>(&self, name: S) -> Option<&FileIndex> {
    self.indexes.get(name.as_ref())
  }
}

impl Default for AppConfig {
  fn default() -> Self {
    let config_file_path = std::env::var("WINDEX_CONF")
      .unwrap_or_else(|_err| "config.yml".to_string());

    let config_file = std::fs::read_to_string(config_file_path);
    let config_file = &config_file.unwrap();

    let config = serde_yaml::from_str(&config_file);

    config.unwrap()
  }
}
