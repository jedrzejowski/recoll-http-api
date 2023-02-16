use crate::config::{read_env_config, ENV_PREFIX};
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
    let mut repo = IndexRepo {
      indexes: HashMap::new(),
    };

    for i in 0.. {
      let prefix = format!("INDEX_{}", i);

      match read_env_config::<FileIndex>(&prefix) {
        Ok(index) => {
          log::info!("resolved index with name '{}'", &index.name);
          repo.indexes.insert(index.name.clone(), index);
        }
        Err(_) => {
          log::info!("stoping resolving indexes");
          break;
        }
      }
    }

    repo
  }
}
