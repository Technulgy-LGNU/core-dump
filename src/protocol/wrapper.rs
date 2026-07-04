use crate::protocol::helpers::{crc32, verify_crc32};
use anyhow::{bail, Context};

const LENGTH_BYTES: usize = size_of::<u32>();
const CRC32_BYTES: usize = size_of::<u32>();
const HEADER_BYTES: usize = LENGTH_BYTES;
const FOOTER_BYTES: usize = CRC32_BYTES;

/// The Wrapper for incoming and outgoing messages
#[derive(Debug, Clone)]
pub struct MessageWrapper {
  /// Message length in bytes
  pub length: u32,

  // Actual data
  pub data: Vec<u8>,

  /// CRC32 Hash
  pub crc32: u32,
}

impl MessageWrapper {
  #[inline]
  pub fn encode(&mut self) -> anyhow::Result<Vec<u8>> {

    self.length = self
      .data
      .len()
      .try_into()
      .context("message wrapper payload is too large for a u32 length field")?;
    self.crc32 = crc32(&self.data);

    let mut encoded = Vec::with_capacity(HEADER_BYTES + self.data.len() + FOOTER_BYTES);
    encoded.extend_from_slice(&self.length.to_le_bytes());
    encoded.extend_from_slice(&self.data);
    encoded.extend_from_slice(&self.crc32.to_le_bytes());
    Ok(encoded)
  }

  #[inline]
  pub fn decode(data: Vec<u8>) -> anyhow::Result<MessageWrapper> {
    if data.len() < HEADER_BYTES + FOOTER_BYTES {
      bail!(
        "message wrapper is too short: got {} bytes, need at least {}",
        data.len(),
        HEADER_BYTES + FOOTER_BYTES
      );
    }

    let length = u32::from_le_bytes(
      data[..HEADER_BYTES]
        .try_into()
        .context("message wrapper is missing its length field")?,
    );
    let payload_len = length as usize;
    let expected_len = HEADER_BYTES
      .checked_add(payload_len)
      .and_then(|len| len.checked_add(FOOTER_BYTES))
      .context("message wrapper length overflow")?;
    if data.len() != expected_len {
      bail!(
        "message wrapper length mismatch: header says {} payload bytes, frame has {} bytes",
        payload_len,
        data.len()
      );
    }

    let payload_end = HEADER_BYTES + payload_len;
    let payload = data[HEADER_BYTES..payload_end].to_vec();
    let crc32 = u32::from_le_bytes(
      data[payload_end..expected_len]
        .try_into()
        .context("message wrapper is missing its crc32 field")?,
    );

    if !verify_crc32(&payload, crc32) {
      bail!("message wrapper crc32 mismatch");
    }

    Ok(MessageWrapper {
      length,
      data: payload,
      crc32,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wrapper_round_trip() {
    let payload = vec![1, 2, 3, 4, 5];
    let mut wrapper = MessageWrapper {
      length: 0,
      data: payload.clone(),
      crc32: 0,
    };

    let encoded = wrapper.encode().unwrap();
    let decoded = MessageWrapper::decode(encoded).unwrap();

    assert_eq!(wrapper.length, payload.len() as u32);
    assert_eq!(decoded.length, payload.len() as u32);
    assert_eq!(decoded.data, payload);
    assert_eq!(decoded.crc32, crc32(&decoded.data));
  }

  #[test]
  fn wrapper_rejects_bad_crc() {
    let mut wrapper = MessageWrapper {
      length: 0,
      data: vec![1, 2, 3],
      crc32: 0,
    };
    let mut encoded = wrapper.encode().unwrap();
    encoded[HEADER_BYTES] ^= 0xff;

    assert!(MessageWrapper::decode(encoded).is_err());
  }

  #[test]
  fn wrapper_rejects_length_mismatch() {
    let mut wrapper = MessageWrapper {
      length: 0,
      data: vec![1, 2, 3],
      crc32: 0,
    };
    let mut encoded = wrapper.encode().unwrap();
    encoded.pop();

    assert!(MessageWrapper::decode(encoded).is_err());
  }
}
