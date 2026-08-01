//! CrashPilot's high-level robot protocol.
//!
//! These messages used to carry a redundant `Cp` prefix in every type name.
//! Keeping them in this module makes their ownership clear without colliding
//! with the SSL protocol types in [`super`].

use super::{Referee, SslWrapperPacket, TrackerWrapperPacket};

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Ball {
  #[prost(message, required, tag = "1")]
  pub pos: Vector2,
  #[prost(message, optional, tag = "2")]
  pub vel: Option<Vector2>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TrackedRobot {
  #[prost(uint32, required, tag = "1")]
  pub robot_id: u32,
  #[prost(message, required, tag = "2")]
  pub pos: Vector2,
  #[prost(int32, required, tag = "3")]
  pub orientation: i32,
  #[prost(message, optional, tag = "4")]
  pub vel: Option<Vector2>,
  #[prost(uint32, required, tag = "5")]
  pub visibility: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Vector2 {
  #[prost(int32, required, tag = "1")]
  pub x: i32,
  #[prost(int32, required, tag = "2")]
  pub y: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Robot {
  #[prost(uint32, required, tag = "1")]
  pub robot_id: u32,
  #[prost(double, required, tag = "2")]
  pub timestamp: f64,
  #[prost(uint32, required, tag = "3")]
  pub packet_id: u32,
  #[prost(message, required, tag = "4")]
  pub ball: Ball,
  #[prost(message, repeated, tag = "5")]
  pub robots_yellow: Vec<TrackedRobot>,
  #[prost(message, repeated, tag = "6")]
  pub robots_blue: Vec<TrackedRobot>,
  #[prost(message, required, tag = "7")]
  pub cmd: Command,
  #[prost(message, required, tag = "8")]
  pub infos: Infos,
}

#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Command {
  #[prost(enumeration = "State", required, tag = "1")]
  pub state: i32,
  #[prost(enumeration = "Task", required, tag = "2")]
  pub task: i32,
  #[prost(message, optional, tag = "3")]
  pub pos: Option<Vector2>,
  #[prost(uint32, optional, tag = "4")]
  pub speed: Option<u32>,
  #[prost(uint32, optional, tag = "5")]
  pub orientation: Option<u32>,
  #[prost(uint32, optional, tag = "6")]
  pub kick_orient: Option<u32>,
  #[prost(uint32, optional, tag = "7")]
  pub kick_speed: Option<u32>,
  #[prost(uint32, optional, tag = "8")]
  pub enemy_id: Option<u32>,
  #[prost(bool, optional, tag = "9")]
  pub raw: Option<bool>,
  #[prost(bool, optional, tag = "10")]
  pub inwall: Option<bool>,
  #[prost(uint32, repeated, packed = "false", tag = "11")]
  pub ignore_robots: Vec<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Infos {
  #[prost(bool, required, tag = "1")]
  pub team_color: bool,
  #[prost(bool, required, tag = "2")]
  pub team_site: bool,
  #[prost(uint32, required, tag = "3")]
  pub width: u32,
  #[prost(uint32, required, tag = "4")]
  pub height: u32,
  #[prost(uint32, required, tag = "5")]
  pub runoff_width: u32,
  #[prost(uint32, required, tag = "6")]
  pub penalty_area_width: u32,
  #[prost(uint32, required, tag = "7")]
  pub penalty_area_height: u32,
  #[prost(uint32, required, tag = "8")]
  pub goal_width: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
  Unspecified = 0,
  Halt = 1,
  Stop = 2,
  Free = 3,
  Goalie = 4,
  Substitute = 5,
}

impl State {
  pub const fn as_str_name(self) -> &'static str {
    match self {
      Self::Unspecified => "STATE_UNSPECIFIED",
      Self::Halt => "STATE_HALT",
      Self::Stop => "STATE_STOP",
      Self::Free => "STATE_FREE",
      Self::Goalie => "STATE_GOALIE",
      Self::Substitute => "STATE_SUBSTITUTE",
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Task {
  Unspecified = 0,
  Pos = 1,
  Kick = 2,
  Chip = 3,
  RecKick = 4,
  Steal = 5,
  Dribble = 6,
  PosBall = 7,
  Block = 8,
  Kickoff = 9,
  Freekick = 11,
}

impl Task {
  pub const fn as_str_name(self) -> &'static str {
    match self {
      Self::Unspecified => "TASK_UNSPECIFIED",
      Self::Pos => "TASK_POS",
      Self::Kick => "TASK_KICK",
      Self::Chip => "TASK_CHIP",
      Self::RecKick => "TASK_REC_KICK",
      Self::Steal => "TASK_STEAL",
      Self::Dribble => "TASK_DRIBBLE",
      Self::PosBall => "TASK_POS_BALL",
      Self::Block => "TASK_BLOCK",
      Self::Kickoff => "STATE_KICKOFF",
      Self::Freekick => "STATE_FREEKICK",
    }
  }
}

/// Data published by CrashPilot to its interface.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceOutput {
  #[prost(message, optional, tag = "1")]
  pub vision_raw: Option<SslWrapperPacket>,
  #[prost(message, optional, tag = "2")]
  pub vision_tracked: Option<TrackerWrapperPacket>,
  #[prost(message, optional, tag = "3")]
  pub gc_data: Option<Referee>,
  #[prost(message, repeated, tag = "4")]
  pub robot_commands: Vec<Robot>,
  #[prost(message, optional, tag = "5")]
  pub cp_gamephase: Option<GamePhase>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GamePhase {
  #[prost(enumeration = "game_phase::Phase", optional, tag = "1")]
  pub game_phase: Option<i32>,
  #[prost(enumeration = "game_phase::PrepPhase", optional, tag = "2")]
  pub prep_phase: Option<i32>,
}

pub mod game_phase {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum Phase {
    UnknownGamePhase = 0,
    Halted = 1,
    Stopped = 2,
    Running = 3,
    Timeout = 4,
    BallPlacement = 5,
  }

  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum PrepPhase {
    UnknownPrepPhase = 0,
    OffensiveKickoff = 1,
    DefensiveKickoff = 2,
    OffensivePenalty = 3,
    DefensivePenalty = 4,
    OffensiveFreeKick = 5,
    DefensiveFreeKick = 6,
  }
}

/// Commands sent by the interface to CrashPilot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceInput {
  #[prost(message, repeated, tag = "1")]
  pub robot_commands: Vec<InterfaceRobotCommand>,
  #[prost(message, required, tag = "2")]
  pub interface_command: InterfaceCommand,
}

#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterfaceRobotCommand {
  #[prost(uint32, required, tag = "1")]
  pub robot_id: u32,
  #[prost(message, required, tag = "2")]
  pub command: Command,
}

#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterfaceCommand {
  #[prost(enumeration = "Mode", required, tag = "1")]
  pub mode: i32,
  #[prost(message, required, tag = "2")]
  pub manual: InterfaceManual,
  #[prost(message, required, tag = "3")]
  pub game: InterfaceGame,
  #[prost(message, required, tag = "4")]
  pub test: InterfaceTest,
  #[prost(bool, required, tag = "5")]
  pub side: bool,
  #[prost(bool, required, tag = "6")]
  pub team_color: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterfaceManual {
  #[prost(bool, required, tag = "1")]
  pub enable_testfield: bool,
  #[prost(uint32, required, tag = "2")]
  pub testfield: u32,
  #[prost(bool, required, tag = "3")]
  pub ball_tracked: bool,
  #[prost(bool, required, tag = "4")]
  pub gc_data: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterfaceGame {
  #[prost(bool, required, tag = "1")]
  pub running: bool,
  #[prost(uint32, required, tag = "4")]
  pub goalkeeper_id: u32,
  #[prost(uint32, required, tag = "5")]
  pub max_speed: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterfaceTest {
  #[prost(enumeration = "Tests", required, tag = "2")]
  pub test: i32,
  #[prost(uint32, repeated, packed = "false", tag = "3")]
  pub robot_ids: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Tests {
  None = 0,
  BallControl = 1,
  Dribbler = 2,
  Kicker = 3,
  GoalShoot = 4,
  Goalie = 5,
  GoalieAndShoot = 6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Mode {
  Manual = 0,
  Game = 1,
  Test = 2,
}

#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RobotFeedback {
  #[prost(uint32, required, tag = "1")]
  pub robot_id: u32,
  #[prost(uint32, optional, tag = "2")]
  pub battery_voltage: Option<u32>,
  #[prost(uint32, optional, tag = "3")]
  pub current: Option<u32>,
  #[prost(bool, required, tag = "4")]
  pub kicker_ready: bool,
  #[prost(bool, required, tag = "5")]
  pub has_ball: bool,
  #[prost(bool, optional, tag = "6")]
  pub has_error: Option<bool>,
  #[prost(bool, optional, tag = "7")]
  pub acting: Option<bool>,
  #[prost(uint32, optional, tag = "8")]
  pub last_rec_packet: Option<u32>,
  #[prost(double, required, tag = "9")]
  pub timestamp: f64,
}
