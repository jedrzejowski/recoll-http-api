mod command;
mod config;
mod deserialize;
mod file_index;
mod index_repo;
mod recollq_output;
mod search_results;

use crate::index_repo::IndexRepo;
use anyhow::Result;
use dotenv::dotenv;
use futures::{io::BufReader, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().ok();
  env_logger::init();

  let config = IndexRepo::default();

  for (name, file_index) in &config.indexes {
    let mut child = file_index.spawn_update_process().await?;

    let mut lines = BufReader::new(child.stderr.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
      println!("{} | {}", name, line?);
    }
  }

  Ok(())
}
