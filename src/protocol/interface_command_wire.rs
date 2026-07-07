//? Size of message:
//?   - Size of message:  Bytes
//?
//? This is all without wrapper message

//?
//? This is the message from the interface to
//? the Crashpilot.
//? It uses the command wrapper, to send commands
//? to the Crashpilot (because we no longer use task stuff)
//?
//? There is also a game section starting the game mode and
//? parameters

use crate::protocol::crashpilot_command_wrapper::CrashpilotCommand;
use serde::{Deserialize, Serialize};

/// Interface Command Wire
/// Message from the Interface to the Crashpilot
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterfaceCommandWire {
  pub timestamp: u64,

  /// Flags
  ///   - Bit 1: Halt / Running
  ///   - Bit 2: Mode Test / Game
  ///   - Bit 3: Enable GameController
  ///   - Bit 4: Manuel Side (0 x+; 1 x-)
  ///   - Bit 5: Manual Color (0 Yellow; 1 Blue)
  ///   - Bit 6:
  ///   - Bit 7:
  ///   - Bit 8:
  ///   - Bit 9:
  ///   - Bit 10:
  ///   - Bit 11:
  ///   - Bit 12:
  ///   - Bit 13:
  ///   - Bit 14:
  ///   - Bit 15:
  ///   - Bit 16:
  pub flags: u16,

  /// Flags for the robot command
  ///   - Bit 1: Enable
  ///   - Bit 2:
  ///   - Bit 3:
  ///   - Bit 4:
  ///   - Bit 5:
  ///   - Bit 6:
  ///   - Bit 7:
  ///   - Bit 8: Shutdown
  pub robot_flags: [Option<u8>; 12],
  pub robot_command: [Option<CrashpilotCommand>; 12],
}

impl InterfaceCommandWire {
  /// JSON messages are variable-length. This is retained for callers that
  /// already refer to the old fixed-size placeholder.
  pub const ENCODED_LEN: usize = 0;

  /// Encodes this command frame as JSON bytes for the interface transport.
  #[inline]
  pub fn encode(&self) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec(self)
  }

  /// Encodes this command frame as a JSON string.
  #[inline]
  pub fn encode_string(&self) -> serde_json::Result<String> {
    serde_json::to_string(self)
  }

  /// Decodes an InterfaceCommandWire from JSON bytes.
  #[inline]
  pub fn decode(data: &[u8]) -> serde_json::Result<Self> {
    serde_json::from_slice(data)
  }

  /// Decodes an InterfaceCommandWire from a JSON string.
  #[inline]
  pub fn decode_str(data: &str) -> serde_json::Result<Self> {
    serde_json::from_str(data)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::vec::types::Vec2;

  #[test]
  fn roundtrips_interface_command_json() {
    let command = InterfaceCommandWire {
      timestamp: 123,
      flags: 0b101,
      robot_flags: std::array::from_fn(|index| if index == 0 { Some(0b1000_0001) } else { None }),
      robot_command: std::array::from_fn(|index| match index {
        0 => Some(CrashpilotCommand::DriveToPos(1, true, Vec2::new(100, -200))),
        1 => Some(CrashpilotCommand::KickGoal(2, false)),
        _ => None,
      }),
    };

    let encoded = command.encode().unwrap();
    let decoded = InterfaceCommandWire::decode(&encoded).unwrap();

    assert_eq!(decoded, command);
  }

  #[test]
  fn decodes_frontend_json_command() {
    let json = r#"{
      "timestamp": 42,
      "flags": 3,
      "robot_flags": [1, null, null, null, null, null, null, null, null, null, null, null],
      "robot_command": [
        {"DriveToPos": [0, false, {"x": 1200, "y": -500}]},
        null, null, null, null, null, null, null, null, null, null, null
      ]
    }"#;

    let decoded = InterfaceCommandWire::decode_str(json).unwrap();

    assert_eq!(decoded.timestamp, 42);
    assert_eq!(decoded.robot_flags[0], Some(1));
    assert_eq!(
      decoded.robot_command[0],
      Some(CrashpilotCommand::DriveToPos(
        0,
        false,
        Vec2::new(1200, -500)
      ))
    );
  }
}
