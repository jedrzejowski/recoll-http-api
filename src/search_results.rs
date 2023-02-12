use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult<T> {
  pub rows: Vec<T>,
  #[serde(rename = "totalCount")]
  pub total_count: u32,
}

