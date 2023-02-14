use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use crate::index_repo::{IndexRepo};
use crate::file_index::FileIndexQueryOptions;

#[derive(Debug, Deserialize)]
pub struct MyPath {
  pub index_name: String
}

pub async fn handler(
    path: web::Path<MyPath>,
    payload: web::Query<FileIndexQueryOptions>,
    app_config: web::Data<IndexRepo>,
) -> actix_web::Result<HttpResponse> {
  let MyPath { index_name } = path.into_inner();

  let file_index = app_config.get_index(index_name)
    .ok_or(actix_web::error::ErrorNotFound("not found"))?;

  let results = file_index.query(payload.into_inner())
    .await
    .map_err(|_err| actix_web::error::ErrorInternalServerError("internal server error"))?;


  Ok(HttpResponse::Ok().json(results))
}