extern crate remo;

use std::env;

fn get_test_token() -> String {
  env::var("NATURE_REMO_CLOUD_API_TOKEN").unwrap()
}

#[test]
fn it_create_instance() {
  let token = get_test_token();
  let client = remo::cloud::Client::new(Some(token));
  let user = client.get_user().unwrap();
  assert!(user.nickname.len() > 0);
}
