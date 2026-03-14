# bserde

Crate for doing binary serialization

## Installation

Install the crate as a dependency in your apps's Cargo.toml file like so:

```toml
[dependencies]
bserde = "0.1.1"
```

## Usage

Import [BSerializer](https://docs.rs/bserde/latest/bserde/trait.BSerializer.html), [BDeserialize](https://docs.rs/bserde/latest/bserde/trait.BDeserialize.html) and [BSerializer](https://docs.rs/bserde/latest/bserde/trait.BSerializer.html) then you should be got to go.

And also what you want to serialize:

* [BSerializeEnum](https://docs.rs/bserde/latest/bserde/trait.BSerializeEnum.html)

* [BSerializeStruct](https://docs.rs/bserde/latest/bserde/trait.BSerializeStruct.html)

```rust
use bserde::{BDeserialize, BSerialize, BSerializer};
```

Define a struct to store your config files. You must implement the [BSerialize](https://docs.rs/bserde/latest/bserde/trait.BSerialize.html) and the [BDeserialize](https://docs.rs/bserde/latest/bserde/trait.BDeserialize.html) trait for your own types

If you dont need much control you can use the derive macro: [bserde_derive](https://crates.io/crates/bserde_derive)

## For Example

```rust
use bserde::{BDeserialize, BSerialize, BSerializer};
use bserde::serializer::BinarySerializer;
use std::{
  fs,
  io::{BufReader, Read},
};

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
    s.end()
  }
}

impl BDeserialize for Person {
  type Error = std::io::Error;
  fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
    Ok(Self {
      name: String::deserialize(input)?,
    })
  }
}

fn main() {
  // SERIALIZING

  // Creating the output file
  let out = fs::File::create("test.bin").unwrap();

  // Creating the serializer
  let mut s = BinarySerializer::new();

  // Creating the person
  let p = Person {
    name: String::from("Hello World"),
    age: 34,
  };

  p.serialize(&mut s);
  // Outputting the file
  s.save(out).unwrap();

  // DESERIALIZING

  // Creating the reader
  let mut new_f = BufReader::new(fs::File::open("test.bin").unwrap());

  // Creating the buffer to store the bytes.
  let mut buf = Vec::new();
  new_f.read_to_end(&mut buf).unwrap();

  // Deserializing the struct
  let p = Person::deserialize(&mut buf.as_slice()).unwrap();

  dbg!(&p);
}
```
