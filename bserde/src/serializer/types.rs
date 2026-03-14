use crate::BSerialize;

macro_rules! serialize_ty {
  ($($t:ty => $method:ident),* $(,)?) => {
    $(
      impl BSerialize<'_> for $t {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where
          S: crate::BSerializer {
            serializer.$method(*self)
        }
      }
    )*
  };
}

serialize_ty!(
  u8 => serialize_u8,
  u16 => serialize_u16,
  u32 => serialize_u32,
  u64 => serialize_u64,
  u128 => serialize_u128,

  i8 => serialize_i8,
  i16 => serialize_i16,
  i32 => serialize_i32,
  i64 => serialize_i64,
  i128 => serialize_i128,
);

impl<'a, T> BSerialize<'a> for Vec<T>
where
  T: BSerialize<'a>,
  &'a T: Into<&'a [u8]> + 'a,
{
  fn serialize<S>(&'a self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: crate::BSerializer,
  {
    serializer.serialize_vec(self.as_ref())
  }
}

impl BSerialize<'_> for dyn AsRef<str> {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: crate::BSerializer,
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl BSerialize<'_> for String {
  fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
  where
    S: crate::BSerializer,
  {
    serializer.serialize_str(self)
  }
}
