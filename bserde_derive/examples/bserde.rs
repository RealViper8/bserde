use std::{
  fs,
  io::{BufReader, Read},
};

use _serde::{BDeserialize, BSerialize, BSerializeStruct, BSerializer};
extern crate bserde as _serde;

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
  num: u16,
  num32: u32,
  num64: u64,
  num128: u128,
}

impl BSerialize<'_> for Person {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: _serde::BSerializer,
  {
    let mut s = serializer.serialize_struct("Person", 2)?;
    s.serialize_field("name", &self.name)?;
    s.serialize_field("age", &self.age)?;
    s.serialize_field("num", &self.num)?;
    s.serialize_field("num32", &self.num32)?;
    s.serialize_field("num64", &self.num64)?;
    s.serialize_field("num128", &self.num128)?;
    s.end()
  }
}

impl BDeserialize for Person {
  type Error = std::io::Error;
  fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
    Ok(Self {
      name: String::deserialize(input)?,
      age: u8::deserialize(input)?,
      num: u16::deserialize(input)?,
      num32: u32::deserialize(input)?,
      num64: u64::deserialize(input)?,
      num128: u128::deserialize(input)?,
    })
  }
}

fn main() {
  let out = fs::File::create("test.bin").unwrap();
  let mut s = _serde::serializer::BinarySerializer::new();
  let p = Person {
    name: String::from("Hello"),
    age: 69,
    num: 32891,
    num32: 10 * 12,
    num64: 10 * 10,
    num128: 10_u128.pow(35),
  };

  p.serialize(&mut s);
  s.save(out).unwrap();

  let mut new_f = BufReader::new(fs::File::open("test.bin").unwrap());
  let mut buf = Vec::new();
  new_f.read_to_end(&mut buf).unwrap();
  let p = Person::deserialize(&mut buf.as_slice()).unwrap();
  dbg!(&p);
}
