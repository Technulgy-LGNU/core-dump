use crate::protocol::robot_command_wire::RobotCommandWire;
use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 170 Bytes
//?   - Throughput at 500Hz: 170 Bytes * 500 = 85kB/s
//?   - Throughput at 1000Hz: 170 Bytes * 1000 = 170kB/s
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

  pub seq: u16,

  pub commands: [RobotCommandWire; 12], // Size is 15 Bytes * 12 = 180 Bytes

  pub crc32: u32,
}

impl RobotCommandFrame {
  #[inline]
  pub fn encode(&self) -> [u8; 164] {
    todo!("Implement encoding of RobotCommandFrame to bytes");
  }

  pub fn decode(message: [u8; 164]) -> RobotCommandFrame {
    todo!("Implement decoding of bytes to RobotCommandFrame");
  }
}
