extern crate remo;
mod helpers;

use remo::cloud::RequestBody;

#[test]
fn it_get_authorized_user() {
  let token = helpers::get_test_token();
  let client = remo::cloud::Client::new(Some(token));
  let res = client.get_user().unwrap();
  assert_eq!(res.nickname, "uetchy");
}

#[test]
fn it_get_devices() {
  let token = helpers::get_test_token();
  let client = remo::cloud::Client::new(Some(token));
  let res = client.get_devices().unwrap();
  println!("{:?}", res);
  assert_eq!(res[0].firmware_version, "Remo/1.0.62-gabbf5bd")
}

#[test]
fn it_get_appliances() {
  let token = helpers::get_test_token();
  let client = remo::cloud::Client::new(Some(token));
  let res = client.get_appliances().unwrap();
  println!("{:?}", res);
  assert_eq!(res[0].device.name, "Living Room")
}

#[test]
fn it_update_aircon_settings() {
  let token = helpers::get_test_token();
  let client = remo::cloud::Client::new(Some(token));

  let appliances = client.get_appliances().unwrap();
  let aircon = appliances.iter().find(|&app| app.r#type == "AC").unwrap();

  let mut body = RequestBody::new();
  body.insert("button", "power-off");
  let resp = client.update_aircon_settings(&aircon.id, &body);
  println!("{:?}", resp);
}

#[test]
fn it_fail_to_get_authorized_user() {
  let client = remo::cloud::Client::new(None);
  let res = client.get_user();
  match res {
    Ok(_) => panic!("should not be ok"),
    Err(e) => match e {
      remo::cloud::APIError::AuthError => assert!(true),
      _ => panic!("unintended error"),
    },
  }
}
