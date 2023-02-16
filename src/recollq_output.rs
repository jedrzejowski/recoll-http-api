use crate::deserialize::{deserialize_number_from_string, deserialize_option_number_from_string};
use crate::search_results::SearchResult;
use anyhow::{anyhow, ensure, Result};
use base64::engine::general_purpose::STANDARD as std_base64;
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecollLine {
  pub caption: Option<String>,
  pub r#abstract: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub dbytes: u64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub fbytes: u64,
  #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
  pub dmtime: Option<i64>,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub fmtime: i64,
  pub filename: String,
  #[serde(default)]
  pub ipath: Option<String>,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub mtime: i64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub pcbytes: u64,
  pub mtype: String,
  pub origcharset: String,
  pub rcludi: String,
  pub relevancyrating: String,
  pub sig: String,
  pub url: String,
}

pub fn parse_recollq_output<S: AsRef<str>>(stdout: S) -> Result<SearchResult<RecollLine>> {
  return parse_recollq_output_inner(stdout).map_err(|err| {
    log::error!("parse_recollq_output_inner error: {}", err);
    anyhow!("error while parsing")
  });
}

fn parse_recollq_output_inner<S: AsRef<str>>(stdout: S) -> Result<SearchResult<RecollLine>> {
  let mut rows = vec![];
  let mut line_iterator = stdout.as_ref().split("\n");
  line_iterator.next();

  let line_with_count = line_iterator.next();
  ensure!(line_with_count.is_some(), "line_with_count exists");
  let total_count = line_with_count.unwrap().split(" ").next();
  ensure!(total_count.is_some(), "line_with_count hase first word");
  let total_count = total_count.unwrap().parse();
  ensure!(total_count.is_ok(), "first word line_with_count is number");
  let total_count = total_count.unwrap();

  for line in line_iterator {
    if line.len() == 0 {
      continue;
    }

    let mut map = serde_json::Map::new();

    let mut iter = line.split(" ");
    loop {
      let prop_name = iter.next();
      let prop_value = iter.next();

      if prop_name.is_none() || prop_value.is_none() {
        break;
      }

      let prop_value = std_base64.decode(prop_value.unwrap())
        .map_err(|err| {
          log::error!("error while parsing base64 command: {}", err);
          anyhow::Error::msg(err)
        })?;
      let prop_value = String::from_utf8_lossy(&prop_value);

      map.insert(
        prop_name.unwrap().to_string(),
        serde_json::Value::String(prop_value.to_string()),
      );
    }

    let recoll_line = serde_json::from_value(serde_json::Value::Object(map))?;

    rows.push(recoll_line)
  }

  Ok(SearchResult { total_count, rows })
}
