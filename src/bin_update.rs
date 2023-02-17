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

    let mut reader = BufReader::new(child.stderr.take().unwrap());
    let mut buf = vec![];

    while reader.read_until(b'\n', &mut buf).await? > 0 {
      let line = String::from_utf8_lossy(&buf);
      println!("{} | {}", name, &line[0..(line.len() - 1)]);
      buf.clear();
    }

    let status = child.status().await?;
    println!("{} | {}", name, status);

    match status.code() {
      Some(code) => {
        println!("{} | exited with status code: {}", name, code)
      },
      None => println!("Process terminated by signal"),
    }
  }

  Ok(())
}
