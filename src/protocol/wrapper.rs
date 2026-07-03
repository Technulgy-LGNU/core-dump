// Even tough there are arrays in both incoming and outgoing messages,
// those arrays have fixed length:
//   For example the `other_robots` field. There is a maximum of 32 robots
//   minus `robot_self`, leaving 31 robots to be accounted for

use crate::protocol::helpers::{crc32, verify_crc32};

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
  pub fn encode(&mut self) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(4 + self.data.len() + 4);
    buf.extend_from_slice(&self.length.to_le_bytes());
    buf.extend_from_slice(&self.data);

    // Create CRC32 Hash of the data and append it to the end of the message
    self.crc32 = crc32(&self.data);
    buf.extend_from_slice(&self.crc32.to_le_bytes());
    buf
  }

  #[inline]
  pub fn decode(data: Vec<u8>) -> anyhow::Result<MessageWrapper> {
    // Decode message
    let msg = MessageWrapper {
      length: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
      data: data[4..4 + u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize].to_vec(),
      crc32: u32::from_le_bytes([
        data[4 + u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize],
        data[5 + u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize],
        data[6 + u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize],
        data[7 + u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize],
      ]),
    };

    // Check CRC32
    if verify_crc32(&msg.data, msg.crc32) {
      Ok(msg)
    } else {
      Err(anyhow::anyhow!("CRC check failed"))
    }
  }
}
