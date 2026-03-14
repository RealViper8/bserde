use crate::deserializer::BDeserialize;
use std::io::Error;

macro_rules! deserialize_ty {
  ($($t:ty),* $(,)?) => {
    $(
      impl BDeserialize for $t {
        type Error = Error;
        fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
          let bytes = read(input, size_of::<$t>())?;
          let arr: [u8; size_of::<$t>()] = bytes.try_into().unwrap();
          Ok(<$t>::from_le_bytes(arr))
        }
      }
    )*
  };
}

fn read<'a>(input: &mut &'a [u8], n: usize) -> Result<&'a [u8], Error> {
  if input.len() < n {
    return Err(Error::new(
      std::io::ErrorKind::InvalidData,
      "Unexpected EOF",
    ));
  }

  let (head, tail) = input.split_at(n);
  *input = tail;

  Ok(head)
}

deserialize_ty!(u8);
deserialize_ty!(u16);
deserialize_ty!(u32);
deserialize_ty!(u64);
deserialize_ty!(u128);

deserialize_ty!(i8);
deserialize_ty!(i16);
deserialize_ty!(i32);
deserialize_ty!(i64);
deserialize_ty!(i128);

impl BDeserialize for String {
  type Error = Error;
  fn deserialize(input: &mut &[u8]) -> Result<Self, Self::Error> {
    let len = u16::deserialize(input)?;
    let bytes = read(input, len as usize)?;
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
  }
}
