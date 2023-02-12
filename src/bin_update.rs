mod config;
mod deserialize;
mod recollq_output;
mod search_results;
mod file_index;

use futures::{io::BufReader, prelude::*};
use crate::config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let config = AppConfig::default();
  
  for (name, file_index) in &config.indexes {
    let mut child = file_index.spawn_update_process().await?;

    let mut lines = BufReader::new(child.stderr.take().unwrap()).lines();

    while let Some(line) = lines.next().await {
      println!("{} | {}", name, line?);
    }
  }

  Ok(())
}