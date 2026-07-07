#[inline]
pub fn crc32(data: &[u8]) -> u32 {
  let mut crc: u32 = 0xFFFF_FFFF;

  for &byte in data {
    crc ^= byte as u32;

    for _ in 0..8 {
      if (crc & 1) != 0 {
        crc = (crc >> 1) ^ 0xEDB8_8320;
      } else {
        crc >>= 1;
      }
    }
  }

  !crc
}

#[inline]
pub fn verify_crc32(data: &[u8], checksum: u32) -> bool {
  crc32(data) == checksum
}

pub mod fixint_array {
  pub mod u16_le {
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::ser::SerializeTuple;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S, const N: usize>(values: &[u16; N], serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut tuple = serializer.serialize_tuple(N)?;
      for value in values {
        tuple.serialize_element(&value.to_le_bytes())?;
      }
      tuple.end()
    }

    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u16; N], D::Error>
    where
      D: Deserializer<'de>,
    {
      struct U16ArrayVisitor<const N: usize>;

      impl<'de, const N: usize> Visitor<'de> for U16ArrayVisitor<N> {
        type Value = [u16; N];

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(formatter, "{N} little-endian u16 values")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: SeqAccess<'de>,
        {
          let mut values = Vec::with_capacity(N);
          for index in 0..N {
            let bytes: [u8; 2] = seq
              .next_element()?
              .ok_or_else(|| Error::invalid_length(index, &self))?;
            values.push(u16::from_le_bytes(bytes));
          }

          values
            .try_into()
            .map_err(|_| Error::invalid_length(N, &self))
        }
      }

      deserializer.deserialize_tuple(N, U16ArrayVisitor::<N>)
    }
  }

  pub mod u32_le {
    use serde::de::{Error, SeqAccess, Visitor};
    use serde::ser::SerializeTuple;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S, const N: usize>(values: &[u32; N], serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut tuple = serializer.serialize_tuple(N)?;
      for value in values {
        tuple.serialize_element(&value.to_le_bytes())?;
      }
      tuple.end()
    }

    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u32; N], D::Error>
    where
      D: Deserializer<'de>,
    {
      struct U32ArrayVisitor<const N: usize>;

      impl<'de, const N: usize> Visitor<'de> for U32ArrayVisitor<N> {
        type Value = [u32; N];

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(formatter, "{N} little-endian u32 values")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
          A: SeqAccess<'de>,
        {
          let mut values = Vec::with_capacity(N);
          for index in 0..N {
            let bytes: [u8; 4] = seq
              .next_element()?
              .ok_or_else(|| Error::invalid_length(index, &self))?;
            values.push(u32::from_le_bytes(bytes));
          }

          values
            .try_into()
            .map_err(|_| Error::invalid_length(N, &self))
        }
      }

      deserializer.deserialize_tuple(N, U32ArrayVisitor::<N>)
    }
  }
}
