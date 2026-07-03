use crate::proto::{SslDetectionBall, Team, TrackedBall, TrackedFrame};
use crate::vec::types::Vec2;

/// Internal use generic robot
pub struct Robot {
  pub robot_id: u8,
  /// 0 = Unknown, 1 = Yellow, 2 = Blue
  pub team: u8,

  pub pos: Vec2<f32>,
  pub vel: Option<Vec2<f32>>,

  pub orientation: f32,
  pub angular_vel: Option<f32>,

  pub visibility: u8,
}

impl Default for Robot {
  fn default() -> Robot {
    Robot {
      robot_id: 0,
      team: 0,
      pos: Vec2::new(0.0, 0.0),
      vel: None,
      orientation: 0.0,
      angular_vel: None,
      visibility: 0,
    }
  }
}

impl Robot {
  /// Takes all the robots and converts them to our own robot type
  pub fn new_from_tracked(frame: &TrackedFrame) -> (Vec<Robot>, Vec<Robot>) {
    let mut robots_yellow: Vec<Robot> = Vec::with_capacity(16);
    let mut robots_blue: Vec<Robot> = Vec::with_capacity(16);

    for robot in &frame.robots {
      if let Some(id) = robot.robot_id.id {
        let r = Robot {
          robot_id: id as u8,
          team: if let Some(team) = robot.robot_id.team {
            team as u8
          } else {
            0
          },
          pos: Vec2::new_from_ssl_vec2(robot.pos),
          vel: if let Some(vel) = robot.vel {
            Some(Vec2::new_from_ssl_vec2(vel))
          } else {
            None
          },
          orientation: robot.orientation,
          angular_vel: if let Some(vel_ang) = robot.vel_angular {
            Some(vel_ang)
          } else {
            None
          },
          visibility: if let Some(vis) = robot.visibility {
            vis.round() as u8
          } else {
            0
          },
        };
        if let Some(team) = robot.robot_id.team
          && team == Team::Yellow as i32
        {
          robots_yellow.insert(id as usize, r);
        } else if let Some(team) = robot.robot_id.team
          && team == Team::Blue as i32
        {
          robots_blue.insert(id as usize, r);
        } else {
          println!("Found robot with no team: {:?}", robot.robot_id);
        }
      } else {
        println!("Found robot with no id: {:?}", robot.robot_id);
      }
    }

    (robots_yellow, robots_blue)
  }
}

/// Our Ball
#[derive(Debug, Default, Copy, Clone)]
pub struct Ball {
  pub pos: Vec2<f32>,
  pub vel: Option<Vec2<f32>>,
}

impl Ball {
  pub fn new_from_vis(
    raw: &Vec<SslDetectionBall>,
    tracked: &Vec<TrackedBall>,
    raw_or_tracked: bool,
    test_field: Option<u8>,
  ) -> Option<Ball> {
    match raw_or_tracked {
      false => {
        if raw.is_empty() {
          return None;
        }
        // Raw
        match test_field {
          Some(test_field) => match test_field {
            0 => {
              let balls: Vec<Ball> = raw
                .iter()
                .filter(|ball| ball.x.is_sign_positive())
                .map(|ball| Ball {
                  pos: Vec2::new(ball.x, ball.y),
                  vel: Default::default(),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            1 => {
              let balls: Vec<Ball> = raw
                .iter()
                .filter(|ball| ball.x.is_sign_negative())
                .map(|ball| Ball {
                  pos: Vec2::new(ball.x, ball.y),
                  vel: Default::default(),
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            _ => {
              println!("Wrong Test Mode: 0 = X+, 1 = X-");
              None
            }
          },
          None => Some(Ball {
            pos: Vec2::new(raw[0].x, raw[0].y),
            vel: Default::default(),
          }),
        }
      }
      true => {
        // Tracked

        if tracked.is_empty() {
          return None;
        }
        match test_field {
          Some(test_field) => match test_field {
            0 => {
              let balls: Vec<Ball> = tracked
                .iter()
                .filter(|ball| ball.pos.x.is_sign_positive())
                .map(|ball| Ball {
                  pos: Vec2::new_from_ssl_vec3(ball.pos),
                  vel: if let Some(vel) = ball.vel {
                    Some(Vec2::new_from_ssl_vec3(vel))
                  } else {
                    None
                  },
                })
                .collect();

              if balls.is_empty() {
                return None;
              }
              Some(balls[0])
            }
            1 => {
              let balls: Vec<Ball> = tracked
                .iter()
                .filter(|ball| ball.pos.x.is_sign_negative())
                .map(|ball| Ball {
                  pos: Vec2::new_from_ssl_vec3(ball.pos),
                  vel: if let Some(vel) = ball.vel {
                    Some(Vec2::new_from_ssl_vec3(vel))
                  } else {
                    None
                  },
                })
                .collect();

              if balls.is_empty() {
                return None
              }
              Some(balls[0])
            }
            _ => {
              println!("Wrong Test Mode: 0 = X+, 1 = X-");
              None
            }
          },
          None => Some(Ball {
            pos: Vec2::new_from_ssl_vec3(tracked[0].pos),
            vel: Some(Vec2::new_from_ssl_vec3(tracked[0].vel.unwrap_or_default())),
          }),
        }
      }
    }
  }
}
