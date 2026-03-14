use std::{fs, io};

use bserde::BDeserialize;
use bserde_derive::{BDeserialize, BSerialize};

#[derive(Debug, PartialEq, BSerialize, BDeserialize)]
struct Cfg(String, u32);

fn main() -> io::Result<()> {
  let c = Cfg("Hello, World".into(), 123456789);
  c.save("test.bin")?;

  let file = fs::read("test.bin")?;
  let new_c = Cfg::deserialize(&mut file.as_slice())?;
  assert!(new_c == c, "Invalid");
  Ok(())
}
