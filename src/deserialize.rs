use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
  where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum StringOrInt<T> {
    String(String),
    Number(T),
  }

  match StringOrInt::<T>::deserialize(deserializer)? {
    StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
    StringOrInt::Number(i) => Ok(i),
  }
}

pub fn deserialize_option_number_from_string<'de, T, D>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum NumericOrNull<T> {
    Str(String),
    FromStr(T),
    Null,
  }

  let qq = NumericOrNull::<T>::deserialize(deserializer);

  match qq? {
    NumericOrNull::Str(s) => match s.as_str() {
      "" => Ok(None),
      _ => T::from_str(&s).map(Some).map_err(serde::de::Error::custom),
    },
    NumericOrNull::FromStr(i) => Ok(Some(i)),
    NumericOrNull::Null => Ok(None),
  }
}