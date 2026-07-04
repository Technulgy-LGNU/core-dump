use crate::types::cp_types::{Ball, Robot};
use crate::vec::types::Vec2;
use serde::{Deserialize, Serialize};

/// The message from the Crashpilot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpRobot {
  // Needs to match on the robot
  // is send, because we dont differentiate anymore
  pub robot_id: u8,
  pub timestamp: u64,

  // The Ball, uses our own Ball data. Either from tracked vision or our own vision filter
  pub ball: Option<Ball>,

  // This is the corresponding robot, all other robots are found in other robots
  pub robot: Option<Robot>,

  // All the other robots
  pub other_robots: [Option<Robot>; 31],

  // The actual command
  pub command: [Option<CpCommand>; 10],

  // Infos, like team, side, field config, etc
  pub infos: CpInfos,

  /// Flags to enable//disable stuff on the robot
  ///   - Bit 1: Enable Robot
  ///   - Bit 2: Enable Kicker
  ///   ...
  ///   - Bit 16: Kill robot
  pub flags: u16,
}
impl CpRobot {
  #[inline]
  pub fn encode(&self) -> Vec<u8> {
    todo!("Finish when CpCommand is finished")
  }

  #[inline]
  pub fn decode(_data: Vec<u8>) -> CpRobot {
    todo!("Finish when CpCommand is finished")
  }

  #[inline]
  pub fn enable_robot(&self) -> bool {
    &self.flags & (1 << 0) != 0
  }
  #[inline]
  pub fn enable_kicker(&self) -> bool {
    &self.flags & (1 << 1) != 0
  }
  // ...
  #[inline]
  pub fn kill_robot(&self) -> bool {
    &self.flags & (1 << 15) != 0
  }
}

/// The actual command the robot has to listen to
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CpCommand {
  // Overall states
  pub main_state: MainState,
  pub cmd_state: CmdState,

  // Actual task
  pub cmd_task: Task,

  /// Flags
  ///  - Bit 1: Enable Dribbler
  ///  - Bit 2: Orient to ball
  ///  - Bit 3: Use Raw instead of ORCA
  pub flags: u32,

  // Task specific data
  pub pos: Option<Vec2<f32>>,
  pub orient: Option<u32>,
  pub kick_orient: Option<u32>,
  pub kick_power: Option<u32>,
  pub dribbler_speed: Option<u32>,
}
impl CpCommand {
  #[inline]
  pub fn enable_dribbler(&self) -> bool {
    &self.flags & (1 << 0) != 0
  }
  #[inline]
  pub fn orient_ball(&self) -> bool {
    &self.flags & (1 << 1) != 0
  }
  #[inline]
  pub fn use_raw(&self) -> bool {
    &self.flags & (1 << 2) != 0
  }
}

/// This is one to one the state from the SSL-Game-Controller
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MainState {
  Unknown,
  Halt,
  Stop,
  Running,
}

/// This is the state the CrashPilot determines for the robot, based on the main state
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CmdState {
  Unknown,
  Free,
  Goalie,
}

/// This is the actual task
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Task {}

/// General infos for the robot
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CpInfos {
  /// 0 = Unknown, 1 = Yellow, 2 = Blue
  pub team: u8,
  /// 0 = Unknown, 1 = X+, -1 = X-
  pub side: i8,

  // Actual field config
  // This is in mm
  pub width: u16,
  pub height: u16,
  pub runoff_width: u16,
  pub penalty_area_width: u16,
  pub penalty_area_height: u16,
  pub goal_width: u16,
}
