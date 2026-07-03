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
