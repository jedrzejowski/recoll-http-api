use crate::command::make_command;
use crate::config::BinPaths;
use crate::recollq_output::parse_recollq_output;
use crate::search_results::SearchResult;
use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct FileIndex {
  pub name: String,
  pub display_name: String,
  pub recoll_config_dir: String,
  pub recoll_url_prefix: String,
  pub url_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileIndexQueryOptions {
  filter: String,
  limit: u32,
  offset: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileIndexResultRow {
  pub caption: Option<String>,
  pub filename: String,
  pub url: String,
  #[serde(rename = "smartPath")]
  pub smart_path: Option<String>,
  #[serde(rename = "mimeType")]
  pub mime_type: String,
  pub r#abstract: String,
  #[serde(rename = "modificationTime")]
  pub modification_time: Option<DateTime<Utc>>,
  #[serde(rename = "sizeInBytes")]
  pub size_in_bytes: Option<u64>,
}

impl FileIndex {
  pub async fn query(
    &self,
    options: FileIndexQueryOptions,
  ) -> Result<SearchResult<FileIndexResultRow>> {
    let mut cmd = make_command(BinPaths::recollq());

    cmd.arg("-c").arg(&self.recoll_config_dir);
    cmd.arg("-n").arg(format!("{}-{}", options.offset, options.limit));
    cmd.arg("-F").arg("").arg("-N");

    cmd.arg(options.filter);

    let output = cmd.output().await.map_err(|err| {
      log::error!("error while executing command: {}", err);
      anyhow::anyhow!("{}", err)
    })?;

    if !output.status.success() {
      log::error!("non zero exit: {:?}", output);
      return Err(anyhow::anyhow!("non zero exit"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let results = parse_recollq_output(stdout)?;

    let mut rows = vec![];

    for recoll_line in results.rows {
      let base_url = if recoll_line.url.starts_with(&self.recoll_url_prefix) {
        &recoll_line.url[self.recoll_url_prefix.len()..]
      } else {
        recoll_line.url.as_str()
      };

      let filename = {
        let base_url = Url::parse(&recoll_line.url)?;
        base_url
          .path_segments()
          .ok_or(anyhow!("ill formatted url"))?
          .last()
          .ok_or(anyhow!("ill formatted url"))?
          .to_string()
      };

      let modification_time = Utc.timestamp_opt(recoll_line.mtime, 0).single();

      let smart_path = if let Some(ipath) = recoll_line.ipath {
        ipath.split('|').last().map(|s| format!("{} | {}", base_url, s))
      } else {
        None
      };

      rows.push(FileIndexResultRow {
        caption: recoll_line.caption,
        filename,
        url: format!("{}{}", self.url_prefix, base_url),
        r#abstract: recoll_line.r#abstract,
        modification_time,
        size_in_bytes: Some(recoll_line.fbytes),
        mime_type: recoll_line.mtype,
        smart_path,
      })
    }

    Ok(SearchResult {
      total_count: results.total_count,
      rows: rows,
    })
  }

  pub async fn spawn_update_process(&self) -> Result<async_process::Child> {
    let mut cmd = make_command(BinPaths::recollindex());

    cmd.arg("-c").arg(&self.recoll_config_dir);

    cmd.stderr(async_process::Stdio::piped());
    cmd.stdout(async_process::Stdio::piped());

    Ok(cmd.spawn()?)
  }
}
