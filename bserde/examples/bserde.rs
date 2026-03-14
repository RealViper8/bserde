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
}

impl BSerialize<'_> for Person {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: _serde::BSerializer,
  {
    let mut s = serializer.serialize_struct("Person", 2)?;
    s.serialize_field("name", &self.name)?;
    s.serialize_field("age", &self.age)?;
    s.end()
  }
}

impl BDeserialize for Person {
  type Error = std::io::Error;
  fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
    Ok(Self {
      name: String::deserialize(input)?,
      age: u8::deserialize(input)?,
    })
  }
}

fn main() {
  let out = fs::File::create("test.bin").unwrap();
  let mut s = _serde::serializer::BinarySerializer::new();
  let p = Person {
    name: String::from("Hello, World"),
    age: 34,
  };

  p.serialize(&mut s);
  s.save(out).unwrap();

  let mut new_f = BufReader::new(fs::File::open("test.bin").unwrap());
  let mut buf = Vec::new();
  new_f.read_to_end(&mut buf).unwrap();
  let p = Person::deserialize(&mut buf.as_slice()).unwrap();
  dbg!(&p);
}
