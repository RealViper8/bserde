# bserde_derive

A crate for making it easier to serialize data to binary

## Installation

You will also need bserde !
Install the crate as a dependency in your apps's Cargo.toml file like so:

```toml
[dependencies]
bserde_derive = "0.1.2"
bserde = "0.1.2"
```

## Usage

Import [BDeserialize Macro](https://docs.rs/bserde_derive/latest/bserde_derive/derive.BDeserialize.html) & [BDeserialze from bserde](https://docs.rs/bserde/latest/bserde/trait.BDeserialize.html) and [BSerialize Macro](https://docs.rs/bserde_derive/latest/bserde_derive/derive.BSerialize.html) then you should be got to go

```rust
use bserde_derive::{BDeserialize, BSerialize};
use bserde::BDeserialize;
```

## For Example

```rust
use bserde_derive::{BDeserialize, BSerialize};
use bserde::BDeserialize;
use std::{fs, io};

#[derive(Debug, BSerialize, BDeserialize)]
struct Config {
  name: String,
}

/// Serializing a config struct into a binary file !
fn main() -> io::Result<()> {
  // Saving on the disk
  let s: Config = Config {
    name: "Content".into(),
  };
  s.save("test.bin")?;

  // Reading
  let file: Vec<u8> = fs::read("test.bin")?;
  let s: Config = Config::deserialize(&mut file.as_slice())?;
  assert!(s.name == String::from("Content"), "Name != Content");

  Ok(())
}
```
