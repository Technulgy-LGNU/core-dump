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
    todo!()
  }

  #[inline]
  pub fn decode(data: Vec<u8>) -> anyhow::Result<MessageWrapper> {
    todo!()
  }
}
