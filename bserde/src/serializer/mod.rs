//! # Serializer
//! `serializer` is a crate containing all the `raw` functions for serializing.
//!

pub mod types;

use crate::{BSerialize, BSerializeEnum, BSerializeStruct, BSerializer};
use std::{
  convert::Infallible,
  io::{self, Write},
};

/// # Used for serializing
/// Better to use the derive macro [`bserde_derive::BSerialize`], [`bserde_derive::BDeserialize`] from bserde_derive
#[derive(Debug, Clone)]
pub struct BinarySerializer {
  output: Vec<u8>,
}

#[derive(Debug)]
pub struct BinarySerializerStruct<'a> {
  ser: &'a mut BinarySerializer,
}

#[derive(Debug)]
pub struct BinarySerializerEnum<'a> {
  ser: &'a mut BinarySerializer,
}

impl BSerializeEnum for BinarySerializerEnum<'_> {
  type Error = Infallible;

  fn serialize_variant(&mut self, value: u16) -> Result<(), Self::Error> {
    value.serialize(self.ser)
  }
}

impl BSerializeStruct for BinarySerializerStruct<'_> {
  type Error = Infallible;

  fn serialize_field<'a, T>(
    &mut self,
    _key: impl Into<Option<&'static str>>,
    value: &'a T,
  ) -> Result<(), Self::Error>
  where
    T: ?Sized + BSerialize<'a>,
  {
    value.serialize(self.ser)
  }

  fn end(self) -> Result<(), Self::Error> {
    Ok(())
  }
}

macro_rules! define_func {
    ($($t:ty => $func:ident),* $(,)?) => {
      $(
        fn $func(&mut self, value: $t) -> Result<(), Self::Error> {
          self.output.extend_from_slice(&value.to_le_bytes());
          Ok(())
        }
      )*
    };
}

impl BSerializer for BinarySerializer {
  type Error = Infallible;
  type SerializeStruct<'a> = BinarySerializerStruct<'a>;
  type SerializeEnum<'a> = BinarySerializerEnum<'a>;

  fn new() -> Self {
    Self { output: Vec::new() }
  }

  define_func!(u8 => serialize_u8);
  define_func!(u16 => serialize_u16);
  define_func!(u32 => serialize_u32);
  define_func!(u64 => serialize_u64);
  define_func!(u128 => serialize_u128);

  define_func!(i8 => serialize_i8);
  define_func!(i16 => serialize_i16);
  define_func!(i32 => serialize_i32);
  define_func!(i64 => serialize_i64);
  define_func!(i128 => serialize_i128);

  fn serialize_vec<'a, 'b, T>(&mut self, value: &'b Vec<T>) -> Result<(), Self::Error>
  where
    'b: 'a,
    T: BSerialize<'a>,
    &'a T: Into<&'a [u8]> + 'a,
  {
    self
      .output
      .extend_from_slice(&(value.len() as u16).to_le_bytes());
    for e in value.iter() {
      self.output.extend_from_slice(e.into());
    }
    Ok(())
  }

  fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
    self
      .output
      .extend_from_slice(&(value.len() as u16).to_le_bytes());
    self.output.extend_from_slice(value.as_bytes());
    Ok(())
  }

  fn serialize_struct(
    &mut self,
    _name: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStruct<'_>, Self::Error> {
    Ok(BinarySerializerStruct { ser: self })
  }

  fn serialize_enum(&mut self) -> Result<Self::SerializeEnum<'_>, Self::Error> {
    Ok(BinarySerializerEnum { ser: self })
  }
}

impl BinarySerializer {
  #[inline]
  pub fn save(&self, mut f: impl Write) -> io::Result<()> {
    f.write_all(&self.output)?;
    f.flush()?;
    Ok(())
  }
}
