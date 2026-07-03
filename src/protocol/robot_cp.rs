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
  pub esc_temps: Vec<u32>,
}

impl RobotCp {
  #[inline]
  pub fn encode(&self) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(
      1 + // robot_id
            8 + // timestamp
            8 + // cp_timestamp
            4 + // flags
            4 + // voltage
            4 + // kick_voltage
            4 + // current
            4 + // cm5_temp
            4 + // control_board_temp
            4 + // kick_temp
            (self.esc_temps.len() * 4), // esc_temps | len = 5 => 20 Bytes
    );

    buf.push(self.robot_id);
    buf.extend_from_slice(&self.timestamp.to_le_bytes());
    buf.extend_from_slice(&self.cp_timestamp.to_le_bytes());
    buf.extend_from_slice(&self.flags.to_le_bytes());
    buf.extend_from_slice(&self.voltage.to_le_bytes());
    buf.extend_from_slice(&self.kick_voltage.to_le_bytes());
    buf.extend_from_slice(&self.current.to_le_bytes());
    buf.extend_from_slice(&self.cm5_temp.to_le_bytes());
    buf.extend_from_slice(&self.control_board_temp.to_le_bytes());
    buf.extend_from_slice(&self.kick_temp.to_le_bytes());
    for esc_temp in &self.esc_temps {
      buf.extend_from_slice(&esc_temp.to_le_bytes());
    }

    buf
  }

  #[inline]
  pub fn decode(data: Vec<u8>) -> RobotCp {
    RobotCp {
      robot_id: data[0],
      timestamp: u64::from_le_bytes([
        data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
      ]),
      cp_timestamp: u64::from_le_bytes([
        data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16],
      ]),
      flags: u32::from_le_bytes([data[17], data[18], data[19], data[20]]),
      voltage: u32::from_le_bytes([data[21], data[22], data[23], data[24]]),
      kick_voltage: u32::from_le_bytes([data[25], data[26], data[27], data[28]]),
      current: u32::from_le_bytes([data[29], data[30], data[31], data[32]]),
      cm5_temp: u32::from_le_bytes([data[33], data[34], data[35], data[36]]),
      control_board_temp: u32::from_le_bytes([data[37], data[38], data[39], data[40]]),
      kick_temp: u32::from_le_bytes([data[41], data[42], data[43], data[44]]),
      esc_temps: data[45..]
        .chunks(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect(),
    }
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
