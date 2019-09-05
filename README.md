# [WIP] Nature Remo for Rust

> Nature Remo crate for rust

This project is in heavily development state and not intended for production use.

## Installation

```
cargo add nature-remo
```

## Usage

```rust
extern crate nature_remo;

use nature_remo::cloud;

fn main() {
  let client = cloud::Client::new("<token>");
  let user = client.get_user()?
  println!("Name: {}", user.nickname)
}
```
