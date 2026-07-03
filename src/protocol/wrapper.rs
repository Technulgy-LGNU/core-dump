// Even tough there are arrays in both incoming and outgoing messages,
// those arrays have fixed length:
//   For example the `other_robots` field. There is a maximum of 32 robots
//   minus `robot_self`, leaving 31 robots to be accounted for

/// The Wrapper for incoming and outgoing messages
pub struct MessageWrapper {
  /// Message length in bytes
  pub length: u32,

  // Actual data
  pub data: Vec<u8>,

  /// CRC32 Hash
  pub crc32: u32,
}

impl MessageWrapper {
  pub fn serialize(&self) -> Vec<u8> {
    todo!("Implement serialization for MessageWrapper");
  }

  pub fn deserialize(data: Vec<u8>) -> MessageWrapper {
    todo!("Implement deserialization for MessageWrapper");
  }
}
