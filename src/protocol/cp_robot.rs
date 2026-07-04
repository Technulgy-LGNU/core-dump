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
  pub fn encode(&self) -> anyhow::Result<Vec<u8>> {
    Ok(postcard::to_extend(self, Vec::new())?)
  }

  #[inline]
  pub fn decode(data: &[u8]) -> anyhow::Result<CpRobot> {
    let (message, remaining) = postcard::take_from_bytes(data)?;
    if !remaining.is_empty() {
      anyhow::bail!(
        "cp_robot message has {} trailing bytes after postcard decode",
        remaining.len()
      );
    }

    Ok(message)
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

#[cfg(test)]
mod tests {
  use super::*;

  fn message() -> CpRobot {
    let mut other_robots = [None; 31];
    other_robots[2] = Some(Robot {
      robot_id: 2,
      team: 1,
      pos: Vec2::new(1200.0, -450.0),
      vel: Some(Vec2::new(12.0, -4.0)),
      orientation: 1.25,
      angular_vel: Some(0.5),
      visibility: 90,
    });

    CpRobot {
      robot_id: 7,
      timestamp: 123_456,
      ball: Some(Ball {
        pos: Vec2::new(100.0, 200.0),
        vel: Some(Vec2::new(3.0, 4.0)),
      }),
      robot: None,
      other_robots,
      command: [None; 10],
      infos: CpInfos {
        team: 1,
        side: -1,
        width: 9000,
        height: 6000,
        runoff_width: 300,
        penalty_area_width: 1000,
        penalty_area_height: 2000,
        goal_width: 1000,
      },
      flags: (1 << 0) | (1 << 15),
    }
  }

  #[test]
  fn postcard_round_trip() {
    let message = message();

    let encoded = message.encode().unwrap();
    let decoded = CpRobot::decode(&encoded).unwrap();

    assert_eq!(decoded.robot_id, message.robot_id);
    assert_eq!(decoded.timestamp, message.timestamp);
    assert_eq!(decoded.infos, message.infos);
    assert!(decoded.enable_robot());
    assert!(!decoded.enable_kicker());
    assert!(decoded.kill_robot());

    let ball = decoded.ball.unwrap();
    assert_eq!(ball.pos.x, 100.0);
    assert_eq!(ball.vel.unwrap().y, 4.0);

    let other_robot = decoded.other_robots[2].unwrap();
    assert_eq!(other_robot.robot_id, 2);
    assert_eq!(other_robot.pos.y, -450.0);
    assert_eq!(other_robot.vel.unwrap().x, 12.0);
  }

  #[test]
  fn decode_rejects_trailing_bytes() {
    let mut encoded = message().encode().unwrap();
    encoded.push(0);

    assert!(CpRobot::decode(&encoded).is_err());
  }
}
