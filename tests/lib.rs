extern crate nature_remo;
mod helpers;

#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn it_get_authorized_user() {
  let token = helpers::get_test_token();
  let client = nature_remo::cloud::Client::new(Some(&token));
  let res = client.get_user().unwrap();
  assert_eq!(res.nickname, "uetchy");
}

#[test]
fn it_get_devices() {
  let token = helpers::get_test_token();
  let client = nature_remo::cloud::Client::new(Some(&token));
  let res = client.get_devices().unwrap();
  println!("{:?}", res);
  assert_eq!(res[0].firmware_version, "Remo/1.0.62-gabbf5bd")
}

#[test]
fn it_fail_to_get_authorized_user() {
  let client = nature_remo::cloud::Client::new(None);
  let res = client.get_user();
  match res {
    Ok(_) => panic!("should not be ok"),
    Err(e) => match e {
      nature_remo::cloud::APIError::AuthError => assert!(true),
      _ => panic!("unintended error"),
    },
  }
}
