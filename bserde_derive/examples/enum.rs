use std::{fs, io};
use bserde_derive::{BDeserialize, BSerialize};
use bserde::BDeserialize;

#[derive(BSerialize, BDeserialize, PartialEq, Eq)]
#[allow(unused)]
enum Menu {
  Settings,
  Home,
}

fn main() -> io::Result<()> {
  let m = Menu::Home;
  m.save("test.bin")?;

  let file = fs::read("test.bin")?;
  let new_m = Menu::deserialize(&mut file.as_slice())?;
  assert!(new_m == Menu::Home, "Invalid data");
  Ok(())
}
