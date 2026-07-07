use crate::protocol::robot_command_wire::RobotCommandWire;
use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 188 Bytes
//?   - Throughput at 500Hz: 188 Bytes * 500 = 94kB/s
//?   - Throughput at 1000Hz: 188 Bytes * 1000 = 188kB/s
//?
//? This is all without wrapper message

//? Instead of sending the message for each robot individual,
//? we construct a single message with all the robot commands.
//? This is more efficient and reduces the overhead of sending multiple messages.

/// Frame with all robot commands
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RobotCommandFrame {
  pub version: u8,
  pub frame_type: u8,

  #[serde(with = "postcard::fixint::le")]
  pub seq: u16,

  pub commands: [RobotCommandWire; 12], // Size is 15 Bytes * 12 = 180 Bytes

  #[serde(with = "postcard::fixint::le")]
  pub crc32: u32,
}

impl RobotCommandFrame {
  pub const ENCODED_LEN: usize = 1 + 1 + 2 + (RobotCommandWire::ENCODED_LEN * 12) + 4;

  #[inline]
  pub fn encode(&self) -> [u8; Self::ENCODED_LEN] {
    let mut message = [0; Self::ENCODED_LEN];
    let encoded = postcard::to_slice(self, &mut message)
      .expect("RobotCommandFrame should fit in its fixed postcard buffer");
    debug_assert_eq!(encoded.len(), Self::ENCODED_LEN);
    message
  }

  pub fn decode(message: [u8; Self::ENCODED_LEN]) -> RobotCommandFrame {
    postcard::from_bytes(&message).expect("RobotCommandFrame fixed buffer should decode")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn roundtrips_fixed_size_frame() {
    let frame = RobotCommandFrame {
      version: u8::MAX,
      frame_type: 2,
      seq: u16::MAX,
      commands: std::array::from_fn(|robot_id| RobotCommandWire {
        robot_id: robot_id as u8,
        mode: u8::MAX,
        intent: u16::MAX,
        vx_mmps: i16::MIN,
        vy_mmps: i16::MAX,
        omega_mradps: -1234,
        kick_speed: u16::MAX,
        dribbler_speed: u16::MAX,
        flags: u8::MAX,
      }),
      crc32: u32::MAX,
    };

    let encoded = frame.encode();

    assert_eq!(encoded.len(), RobotCommandFrame::ENCODED_LEN);
    assert_eq!(RobotCommandFrame::decode(encoded), frame);
  }
}
