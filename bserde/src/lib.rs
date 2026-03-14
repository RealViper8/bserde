use std::error::Error;

pub mod serializer;
pub use serializer::types;

pub mod deserializer;

// SERIALIZER

pub trait BSerialize<'a> {
  fn serialize<S>(&'a self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: BSerializer;
}

pub trait BSerializer: Sized {
  type Error: Error;

  type SerializeStruct<'a>: BSerializeStruct<Error = Self::Error>
  where
    Self: 'a;

  type SerializeEnum<'a>: BSerializeEnum<Error = Self::Error>
  where
    Self: 'a;

  fn new() -> Self;

  fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error>;
  fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error>;
  fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error>;
  fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error>;
  fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error>;
  fn serialize_u128(&mut self, value: u128) -> Result<(), Self::Error>;

  fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error>;
  fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error>;
  fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error>;
  fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error>;
  fn serialize_i128(&mut self, value: i128) -> Result<(), Self::Error>;

  fn serialize_vec<'a, 'b, T>(&mut self, value: &'b Vec<T>) -> Result<(), Self::Error>
  where
    'b: 'a,
    T: BSerialize<'a>,
    &'a T: Into<&'a [u8]> + 'a;

  fn serialize_struct(
    &mut self,
    name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStruct<'_>, Self::Error>;

  fn serialize_enum(&mut self) -> Result<Self::SerializeEnum<'_>, Self::Error>;
}

pub trait BSerializeStruct {
  type Error: Error;

  fn serialize_field<'a, T>(
    &mut self,
    key: impl Into<Option<&'static str>>,
    value: &'a T,
  ) -> Result<(), Self::Error>
  where
    T: ?Sized + BSerialize<'a>;

  fn end(self) -> Result<(), Self::Error>;
}

pub trait BSerializeEnum {
  type Error: Error;

  fn serialize_variant(&mut self, value: u16) -> Result<(), Self::Error>;
}

// DESERIALIZER

pub trait BDeserialize: Sized {
  type Error: Error;

  fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error>;
}
