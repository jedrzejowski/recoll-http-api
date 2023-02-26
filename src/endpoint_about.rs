use crate::file_index::FileIndexQueryOptions;
use crate::index_repo::IndexRepo;
use crate::search_results::{Pagination, SearchResult};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct About {
  pub name: String,
  pub git_hash: String,
}

impl Default for About {
  fn default() -> Self {
    About {
      name: env!("CARGO_PKG_NAME").to_string(),
      git_hash: env!("GIT_HASH").to_string(),
    }
  }
}

pub async fn handler() -> actix_web::Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(About::default()))
}
