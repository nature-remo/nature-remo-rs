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

fn main() {
  let client = nature_remo::Cloud::new("<token>");
  let user = client.get_user();
}
```
