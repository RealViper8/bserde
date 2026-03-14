use bserde_derive::{BDeserialize, BSerialize};
use bserde::BDeserialize;
use std::{fs, io};

#[derive(Debug, BSerialize, BDeserialize)]
struct Config {
  name: String,
}

/// Serializing a config struct into a binary file !
fn main() -> io::Result<()> {
  // Saving
  let s = Config {
    name: "Content".into(),
  };
  s.save("test.bin")?;

  // Reading
  let file = fs::read("test.bin")?;
  let s = Config::deserialize(&mut file.as_slice())?;
  assert!(s.name == String::from("Content"), "Name != Content");

  Ok(())
}
