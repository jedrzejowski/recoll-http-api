mod api_key_guard;
mod command;
mod config;
mod deserialize;
mod endpoint_about;
mod endpoint_list;
mod endpoint_query;
mod file_index;
mod index_repo;
mod recollq_output;
mod search_results;

use crate::api_key_guard::ApiKeyGuard;
use crate::config::read_env_config;
use crate::index_repo::IndexRepo;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppHttpConfig {
  #[serde(default = "AppHttpConfig::default_host")]
  pub host: String,
  #[serde(default = "AppHttpConfig::default_port")]
  pub port: u16,
  pub api_key: Option<String>,
}

impl AppHttpConfig {
  fn default_host() -> String {
    "0.0.0.0".to_string()
  }
  fn default_port() -> u16 {
    8080
  }
}

#[actix_web::main]
async fn main() -> Result<()> {
  dotenv().ok();
  env_logger::init();

  let webserver_cfg: AppHttpConfig = read_env_config("HTTP")?;
  log::info!("staring http server on {}:{}", webserver_cfg.host, webserver_cfg.port);

  let index_repo = web::Data::new(IndexRepo::default());
  let api_guard = ApiKeyGuard::default();

  HttpServer::new(move || {
    let mut app = App::new().app_data(index_repo.clone());

    app = app.route("/about", web::get().to(endpoint_about::handler));

    app = app.service(
      web::scope("/indexes")
        .wrap(api_guard.clone())
        .route("/list", web::get().to(endpoint_list::handler))
        .route("/{index_name}/query", web::get().to(endpoint_query::handler)),
    );

    app
  })
  .workers(4)
  .bind((webserver_cfg.host, webserver_cfg.port))?
  .run()
  .await
  .map_err(anyhow::Error::msg)
}
