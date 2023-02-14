use crate::file_index::FileIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexRepo {
  pub indexes: HashMap<String, FileIndex>,
}

impl IndexRepo {
  pub fn get_index<S: AsRef<str>>(&self, name: S) -> Option<&FileIndex> {
    self.indexes.get(name.as_ref())
  }
}

impl Default for IndexRepo {
  fn default() -> Self {
    let config_file_path = std::env::var("WINDEX_REPO")
      .unwrap_or("indexes.yml".to_string());

    let config_file = std::fs::read_to_string(config_file_path);
    let config_file = &config_file.unwrap();

    let config = serde_yaml::from_str(&config_file);

    config.unwrap()
  }
}
