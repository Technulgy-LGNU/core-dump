use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 15 Bytes
//?   - Size for 12 robots: 12 * 15 = 180 Bytes
//?   - Throughput at 500Hz: 180 Bytes * 500 = 90kB/s
//?   - Throughput at 1000Hz: 180 Bytes * 1000 = 180kB/s
//?
//? This is all without wrapper message

/// Robot Control Protocol
/// Sends basic data to each robot
/// to control movement, etc
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RobotCommandWire {
  pub robot_id: u8,
  pub mode: u8,
  #[serde(with = "postcard::fixint::le")]
  pub intent: u16,

  // Velocity in mm/s
  #[serde(with = "postcard::fixint::le")]
  pub vx_mmps: i16,
  #[serde(with = "postcard::fixint::le")]
  pub vy_mmps: i16,
  // Rotation in mrad/s
  #[serde(with = "postcard::fixint::le")]
  pub omega_mradps: i16,

  #[serde(with = "postcard::fixint::le")]
  pub kick_speed: u16,
  #[serde(with = "postcard::fixint::le")]
  pub dribbler_speed: u16,

  /// Flags to execute specific robot stuff
  ///   - Bit 1: Kick
  ///   - Bit 2: Chip
  ///   - Bit 3: Dribbler
  ///   - Bit 4:
  ///   - Bit 5:
  ///   - Bit 6:
  ///   - Bit 7:
  ///   - Bit 8: Shutdown
  pub flags: u8,
}
impl RobotCommandWire {
  pub const ENCODED_LEN: usize = 15;

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotCommandWire should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  #[inline]
  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> Self {
    postcard::from_bytes(&message).expect("RobotCommandWire fixed buffer should decode")
  }

  #[inline]
  pub fn kick(&self) -> bool {
    self.flags & (1 << 0) != 0
  }
  #[inline]
  pub fn chip(&self) -> bool {
    self.flags & (1 << 1) != 0
  }
  #[inline]
  pub fn dribbler(&self) -> bool {
    self.flags & (1 << 2) != 0
  }
  // #[inline]
  // pub fn bit4(&self) -> bool {
  //   self.flags & (1 << 4) != 0
  // }
  // #[inline]
  // pub fn bit5(&self) -> bool {
  //   self.flags & (1 << 5) != 0
  // }
  // #[inline]
  // pub fn bit6(&self) -> bool {
  //   self.flags & (1 << 6) != 0
  // }
  // #[inline]
  // pub fn bit7(&self) -> bool {
  //   self.flags & (1 << 7) != 0
  // }
  #[inline]
  pub fn shutdown(&self) -> bool {
    self.flags & (1 << 7) != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_command() {
    let command = RobotCommandWire {
      robot_id: u8::MAX,
      mode: u8::MAX,
      intent: u16::MAX,
      vx_mmps: i16::MIN,
      vy_mmps: i16::MAX,
      omega_mradps: -1234,
      kick_speed: u16::MAX,
      dribbler_speed: u16::MAX,
      flags: u8::MAX,
    };

    let encoded = command.encode();

    assert_eq!(encoded.len(), RobotCommandWire::ENCODED_LEN);
    assert_eq!(RobotCommandWire::decode(encoded), command);
  }
}
