use crate::file_index::FileIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
use crate::config::{ENV_PREFIX, read_env_config};

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
      indexes: HashMap::new()
    };

    for i in 0.. {
      let prefix = format!("INDEX_{}_", i);

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
