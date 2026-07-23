//? This Wrapper defines basic commands to send from the Interface
//? to the Crashpilot
//?
//? Will be expanded in the future

use crate::vec::types::Vec2;
use serde::{Deserialize, Serialize};

/// Commands for the Crashpilot
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CrashpilotCommand {
  //? Simple tasks
  /// The bool describes if the robot should
  /// use raw or orca movement
  DriveToPos(u8, bool, Vec2<i32>),
  /// This is a simple shoot at goal, no skill
  /// The bool describes, if the robot should chip
  KickGoal(u8, bool),

  //? Skills
  //? The previous commands are just simple commands to test stuff
  //? Skills are the logic building blocks used in the actual game in the end
  /// Performs a coordinated goal kick
  /// There need to be two or more robots selected
  ///   - Two Robots: one robot moves behind ball, the
  ///     other robot moves to one side of the goal
  ///     the pass each other to score the goal
  ///
  ///   - Three robots: Two robots position themselves near
  ///   the goal and then the third robot gets the ball and
  ///   starts the task
  SkillKickGoal([Option<u8>; 3]),
  /// The bool describes if the robot should chip
  /// If Vec2 is some, the receiving robot
  /// should drive to that position, while receiving
  /// the pass
  SkillPassTo([u8; 2], bool, Option<Vec2<i32>>),
  /// Block an enemy
  SkillBlockEnemy(u8),
  /// Protect goal
  /// There can be multiple robots assigned to that command
  SkillProtectGoal(Vec<u8>),
}

pub enum Intent {
  Goalie,
  RecBall,
  KickBall,
  Prepare,
}
