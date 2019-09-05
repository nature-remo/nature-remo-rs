extern crate nature_remo;
mod helpers;

#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn it_get_authorized_user() {
  let token =helpers::get_test_token();
  let client = nature_remo::Cloud::new(Some(&token));
  let response = client.get_user().unwrap();
  assert_eq!(response.nickname, "uetchy");
}