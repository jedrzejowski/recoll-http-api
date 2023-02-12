mod endpoint_query;
mod config;
mod deserialize;
mod recollq_output;
mod search_results;
mod api_key;
mod file_index;

use actix_web::{web, App, HttpServer};
use crate::api_key::ApiKeyGuard;
use crate::config::AppConfig;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init();

  HttpServer::new(|| {
    let app_config = web::Data::new(AppConfig::default());

    let mut app = App::new()
      .app_data(app_config);

    app = app.service(
      web::scope("/indexes")
        .wrap(ApiKeyGuard { api_key: "dupa".to_string() })
        .route("/{index_name}/query", web::get().to(endpoint_query::handler))
    );

    app
  })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}