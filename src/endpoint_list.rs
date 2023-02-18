use crate::file_index::FileIndexQueryOptions;
use crate::index_repo::IndexRepo;
use crate::search_results::{Pagination, SearchResult};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Index {
  pub name: String,
  #[serde(rename = "displayName")]
  pub display_name: String,
}

pub async fn handler(
  pagination: web::Query<Pagination>,
  index_repo: web::Data<IndexRepo>,
) -> actix_web::Result<HttpResponse> {
  let mut result = vec![];

  for (index, (_, file_index)) in index_repo.indexes.iter().enumerate() {
    if !pagination.is_in(index) {
      continue;
    }

    result.push(Index {
      name: file_index.name.clone(),
      display_name: file_index.display_name.clone(),
    })
  }

  let result = SearchResult {
    rows: result,
    total_count: index_repo.indexes.len(),
  };

  Ok(HttpResponse::Ok().json(result))
}
