use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult<T> {
  pub rows: Vec<T>,
  #[serde(rename = "totalCount")]
  pub total_count: u32,
}

impl<T> SearchResult<T> {
  pub fn map_rows<F, R>(self, mapper: F) -> SearchResult<R>
    where F: FnMut(&T) -> R {
    SearchResult {
      rows: self.rows.iter().map(mapper).collect(),
      total_count: self.total_count
    }
  }
}
