pub mod cloud;

#[cfg(test)]
pub mod tests {
  use std::env;
  pub fn get_test_token() -> String {
    env::var("NATURE_REMO_CLOUD_API_TOKEN").unwrap()
  }
}
