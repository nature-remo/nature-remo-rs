# Nature Remo for Rust

[![Crates.io](https://img.shields.io/crates/v/remo)](https://crates.io/crates/remo)
[![Docs.rs](https://docs.rs/remo/badge.svg)](https://docs.rs/remo)
[![Travis (.com)](https://img.shields.io/travis/com/uetchy/nature-remo-rs)](https://travis-ci.com/uetchy/nature-remo-rs)

> Nature Remo API client for Rust.

**WARNING: This project is in heavily development state and not intended for production use.**

## Installation

```
cargo add remo
```

## Usage

```rust
extern crate remo;

use remo::cloud;
use std::env;

fn main() {
  let token = env::var("NATURE_REMO_CLOUD_API_TOKEN");

  let client = cloud::Client::new(token);

  // get user's nickname
  let user = client.get_user().unwrap();
  println!("Name: {}", user.nickname);

  // get room temperature
  let sensor_value = client.get_sensor_value().unwrap();
  println!("Temperature: {}", sensor_value.temperature);

  // update aircon settings
  let appliances = client.get_appliances().unwrap();
  let aircon = appliances.iter().find(|&app| app.r#type == "AC").unwrap();

  let mut params = cloud::RequestBody::new();
  params.insert("operation_mode", "warm");
  params.insert("temperature", "26");
  client.update_aircon_settings(&aircon.id, &params);
  println!("Aircon settings updated: mode(warm), temperature(26)");
}
```

## Related Projects

### [Nature Remo for JavaScript](https://github.com/uetchy/nature-remo)
