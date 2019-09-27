#[allow(dead_code)]
mod optional_string {
  use serde::de;
  use serde::de::{Deserialize, Deserializer};
  use serde::ser::Serializer;
  use std::fmt::Display;
  use std::str::FromStr;

  pub fn serialize<T, S>(value: Option<&T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Display,
    S: Serializer,
  {
    serializer.collect_str(value.unwrap())
  }

  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
  {
    Ok(Some(
      String::deserialize(deserializer)?
        .parse()
        .map_err(de::Error::custom)?,
    ))
  }
}
