use crate::vec::types::Vec2;

#[derive(Debug, Clone, Copy, Default)]
pub struct BallState {
  pub pos: Vec2<f32>,
  pub vel: Vec2<f32>,
  pub stop_pos: Vec2<f32>,
  pub stop_time: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RobotState {
  pub id: u8,
  pub pos: Vec2<f32>,
  pub vel: Vec2<f32>,
  pub heading: f32,
  pub angular_vel: f32,
  pub is_goalie: bool,
}

pub type Robots = [Option<RobotState>; 16];

pub enum Team {
  Own,
  Opp,
}

pub enum GameStage {
  Running,
  Stop,
  Halt,
  BallPlacement(Vec2<f32>, Team),
  PrepareKickoff,
  Kickoff,
  FreeKick,
  Penalty,
  ShootOut(Team),
}

#[derive(Debug, Copy, Clone, Default)]
pub struct World {
  pub own_robots: Robots,
  pub opp_robots: Robots,
  pub ball: BallState,
}

pub struct GameState {
  pub world: World,
  pub stage: GameStage,
}

pub enum Intent {
  Goalie,
  PassTo(u8),
  RecPass,
  KickGoal,
  Block,
  Wall,
  Steal,
  GetBallTurn,
  GetBallBehind,
  Hold,
}

pub struct RobotCommand {
  pub dribbler: bool,
  pub raw_movement: bool,
  pub avoid_ball_collision: bool,
  pub pos: Option<Pos>,
  pub kicker: Kicker,
}

pub struct Pos {
  pub pos: Vec2<f32>,
  pub face: Option<f32>,
  pub speed: Option<u32>,
}

pub enum Kicker {
  None,
  Chip(f32), //dist in mm
  Kick(f32), // dist in mm
}

pub type Commands = [Option<RobotCommand>; 16];

pub trait Ai {
  fn predict(&mut self, state: GameState) -> Commands;

  fn debug(&self) -> String {
    String::new()
  }
}

#[derive(Default)]
pub struct DummyAi;

impl Ai for DummyAi {
  fn predict(&mut self, _state: GameState) -> Commands {
    Commands::default()
  }
}
