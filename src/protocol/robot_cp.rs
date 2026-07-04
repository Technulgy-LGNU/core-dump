/// The message from the Robot to the CrashPilot
/// Contains status mainly status information and some debug
/// data.
#[derive(Debug, Clone)]
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
