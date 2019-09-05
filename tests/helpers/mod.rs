use std::env;

// #[allow(dead_code)]
pub fn get_test_token() -> String {
  env::var("NATURE_REMO_CLOUD_API_TOKEN").unwrap()
}
