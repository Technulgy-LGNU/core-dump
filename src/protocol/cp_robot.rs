use crate::types::cp_types::{Ball, Robot};

/// The message from the Crashpilot
#[derive(Debug, Clone)]
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
    pub other_robots: Vec<Robot>,

    // The actual command
    pub command: Option<CpCommand>,

    // Infos, like team, side, field config, etc
    pub infos: CpInfos,
}
impl CpRobot {
    pub fn encode(&self) -> Vec<u8> {
        todo!("Finish when CpCommand is finished")
    }

    pub fn decode(_data: Vec<u8>) -> CpRobot {
        todo!("Finish when CpCommand is finished")
    }
}

/// The actual command the robot has to listen to
#[derive(Debug, Copy, Clone)]
pub struct CpCommand {
    // Overall states
    pub main_state: MainState,
    pub cmd_state: CmdState,


}
impl CpCommand {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::with_capacity(1 + 1);
        buf.push(self.main_state as u8);
        buf.push(self.cmd_state as u8);
        buf
    }

    pub fn decode(data: Vec<u8>) -> CpCommand {
        CpCommand {
            main_state: match data[0] {
                0 => MainState::Unknown,
                1 => MainState::Halt,
                2 => MainState::Stop,
                3 => MainState::Running,
                _ => MainState::Unknown,
            },
            cmd_state: match data[1] {
                0 => CmdState::Unknown,
                1 => CmdState::Free,
                2 => CmdState::Goalie,
                _ => CmdState::Unknown,
            },
        }
    }
}

/// This is one to one the state from the SSL-Game-Controller
#[derive(Debug, Copy, Clone)]
pub enum MainState {
    Unknown,
    Halt,
    Stop,
    Running,
}

/// This is the state the CrashPilot determines for the robot, based on the main state
#[derive(Debug, Copy, Clone)]
pub enum CmdState {
    Unknown,
    Free,
    Goalie,
}

/// This is the actual task

/// General infos for the robot
#[derive(Debug, Copy, Clone)]
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
impl CpInfos {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::<u8>::with_capacity(
            1 + // team
            1 + // side
            2 + // width
            2 + // height
            2 + // runoff_width
            2 + // penalty_area_width
            2 + // penalty_area_height
            2   // goal_width
        );
        buf.push(self.team);
        buf.push(self.side as u8);
        buf.extend_from_slice(&self.width.to_le_bytes());
        buf.extend_from_slice(&self.height.to_le_bytes());
        buf.extend_from_slice(&self.runoff_width.to_le_bytes());
        buf.extend_from_slice(&self.penalty_area_width.to_le_bytes());
        buf.extend_from_slice(&self.penalty_area_height.to_le_bytes());
        buf.extend_from_slice(&self.goal_width.to_le_bytes());
        buf
    }

    pub fn decode(data: Vec<u8>) -> CpInfos {
        CpInfos {
            team: data[0],
            side: data[1] as i8,
            width: u16::from_le_bytes([data[2], data[3]]),
            height: u16::from_le_bytes([data[4], data[5]]),
            runoff_width: u16::from_le_bytes([data[6], data[7]]),
            penalty_area_width: u16::from_le_bytes([data[8], data[9]]),
            penalty_area_height: u16::from_le_bytes([data[10], data[11]]),
            goal_width: u16::from_le_bytes([data[12], data[13]]),
        }
    }
}
