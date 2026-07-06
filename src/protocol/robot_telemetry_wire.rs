use serde::{Deserialize, Serialize};

//? Size of message:
//?   - Size of message: 16 Bytes
//?   - Size for 12 robots: 16 * 12 = 192 Bytes
//?   - Throughput at 500Hz: 192 Bytes * 500 = 96kB/s
//?   - Throughput at 1000Hz: 192 Bytes * 1000 = 192kB/s
//?
//? This is all without wrapper message

/// This is a small debug message from the robot to the CrashPilot
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RobotTelemetryWire {
  pub robot_id: u8,
  pub status: u8,

  pub seq_seen: u32,

  // Data from the onboard imu
  pub vx_mmps: i16,
  pub vy_mmps: i16,
  pub omega_mradps: i16,

  pub battery_mv: u8,
  pub current: u8,
  pub capacitor_v: u8,

  /// Flags from the robot to the CrashPilot
  ///   - Bit 1: Has Ball
  ///   - Bit 2: Reserved
  ///   - Bit 3: Reserved
  ///   - Bit 4: Reserved
  ///   - Bit 5: Fault ESC 1
  ///   - Bit 6: Fault ESC 2
  ///   - Bit 7: Fault ESC 3
  ///   - Bit 8: Fault ESC 4
  ///   - Bit 9: Fault ESC 5
  ///   - Bit 10: Fault Kicker
  ///   - Bit 11: Fault Vision
  ///   - Bit 12: Fault Wi-Fi
  ///   - Bit 13: Fault Battery
  ///   - Bit 14:
  ///   - Bit 15:
  ///   - Bit 16: Shutting down
  pub flags: u16,
}

impl RobotTelemetryWire {
  #[inline]
  pub fn encode(&self) -> [u8; 16] {
    todo!("Implement encoding of RobotTelemetryWire to bytes");
  }

  #[inline]
  pub fn decode(message: [u8; 16]) -> Self {
    todo!("Implement decoding of bytes to RobotTelemetryWire");
  }

  #[inline]
  pub fn has_ball(&self) -> bool {
    self.flags & (1 << 0) != 0
  }
  // #[inline]
  // pub fn reserved_bit_2(&self) -> bool {
  //   self.flags & (1 << 1) != 0
  // }
  // #[inline]
  // pub fn reserved_bit_3(&self) -> bool {
  //   self.flags & (1 << 2) != 0
  // }
  // #[inline]
  // pub fn reserved_bit_4(&self) -> bool {
  //   self.flags & (1 << 3) != 0
  // }
  #[inline]
  pub fn esc_fault(&self) -> Option<u8> {
    for i in 0..5 {
      if self.flags & (1 << (4 + i)) != 0 {
        return Some(i + 1);
      }
    }
    None
  }
  #[inline]
  pub fn fault_kicker(&self) -> bool {
    self.flags & (1 << 9) != 0
  }
  #[inline]
  pub fn fault_vision(&self) -> bool {
    self.flags & (1 << 10) != 0
  }
  #[inline]
  pub fn fault_wifi(&self) -> bool {
    self.flags & (1 << 11) != 0
  }
  #[inline]
  pub fn fault_battery(&self) -> bool {
    self.flags & (1 << 12) != 0
  }
  // #[inline]
  // pub fn bit14(&self) -> bool {
  //   self.flags & (1 << 13) != 0
  // }
  // #[inline]
  // pub fn bit15(&self) -> bool {
  //   self.flags & (1 << 14) != 0
  // }
  #[inline]
  pub fn is_shutting_down(&self) -> bool {
    self.flags & (1 << 15) != 0
  }
}
