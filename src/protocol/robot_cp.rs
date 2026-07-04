use serde::{Deserialize, Serialize};

/// The message from the Robot to the CrashPilot
/// Contains status mainly status information and some debug
/// data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotCp {
  pub robot_id: u8,
  pub timestamp: u64,
  pub cp_timestamp: u64,

  /// Flags for general states
  ///   - Bit 1: Robot Ready
  ///   - Bit 2: Robot Enabled
  ///   - Bit 3: Kicker Ready
  ///   - Bit 4: Has Ball
  ///
  /// rest for other stuff
  pub flags: u32,

  // infos
  // All of these are multiplied by 100 to have one decimal precision without using floats
  pub voltage: u32,
  pub kick_voltage: u32,
  pub current: u32,
  // Temperatures of different parts on the robot
  pub cm5_temp: u32,
  pub control_board_temp: u32,
  pub kick_temp: u32,
  // Temperatures of the ESCs, in order of the motors (1-4 + Dribbler)
  pub esc_temps: [u32; 5],
}

impl RobotCp {
  #[inline]
  pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
    Ok(postcard::to_extend(self, Vec::new())?)
  }

  #[inline]
  pub fn decode(data: &[u8]) -> anyhow::Result<RobotCp> {
    let (message, remaining) = postcard::take_from_bytes(data)?;
    if !remaining.is_empty() {
      anyhow::bail!(
        "robot_cp message has {} trailing bytes after postcard decode",
        remaining.len()
      );
    }

    Ok(message)
  }

  // Flag checks
  #[inline]
  pub fn robot_read(&self) -> bool {
    self.flags & (1 << 0) != 0
  }
  #[inline]
  pub fn robot_enabled(&self) -> bool {
    self.flags & (1 << 1) != 0
  }
  #[inline]
  pub fn kicker_ready(&self) -> bool {
    self.flags & (1 << 2) != 0
  }
  #[inline]
  pub fn has_ball(&self) -> bool {
    self.flags & (1 << 3) != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn message() -> RobotCp {
    RobotCp {
      robot_id: 3,
      timestamp: 987_654,
      cp_timestamp: 123_456,
      flags: 0b1111,
      voltage: 2410,
      kick_voltage: 18000,
      current: 430,
      cm5_temp: 480,
      control_board_temp: 390,
      kick_temp: 410,
      esc_temps: [310, 320, 330, 340, 350],
    }
  }

  #[test]
  fn postcard_round_trip() {
    let message = message();

    let encoded = message.encode().unwrap();
    let decoded = RobotCp::decode(&encoded).unwrap();

    assert_eq!(decoded.robot_id, message.robot_id);
    assert_eq!(decoded.timestamp, message.timestamp);
    assert_eq!(decoded.cp_timestamp, message.cp_timestamp);
    assert_eq!(decoded.voltage, message.voltage);
    assert_eq!(decoded.kick_voltage, message.kick_voltage);
    assert_eq!(decoded.current, message.current);
    assert_eq!(decoded.esc_temps, message.esc_temps);
    assert!(decoded.robot_read());
    assert!(decoded.robot_enabled());
    assert!(decoded.kicker_ready());
    assert!(decoded.has_ball());
  }

  #[test]
  fn decode_rejects_trailing_bytes() {
    let mut encoded = message().encode().unwrap();
    encoded.push(0);

    assert!(RobotCp::decode(&encoded).is_err());
  }
}
