use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult<T> {
  pub rows: Vec<T>,
  #[serde(rename = "totalCount")]
  pub total_count: usize,
}

impl<T> SearchResult<T> {
  pub fn slice_from_vec(vector: Vec<T>, offset: usize, limit: usize) -> SearchResult<T>
  where
    T: Clone,
  {
    SearchResult {
      total_count: vector.len(),
      rows: vector[offset..offset + limit].to_vec(),
    }
  }

  pub fn map_rows<F, R>(self, mapper: F) -> SearchResult<R>
  where
    F: FnMut(&T) -> R,
  {
    SearchResult {
      rows: self.rows.iter().map(mapper).collect(),
      total_count: self.total_count,
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
  pub limit: usize,
  pub offset: usize,
}

impl Pagination {
  pub fn is_in(&self, num: usize) -> bool {
    return num >= self.offset && num < self.offset + self.limit;
  }
}
