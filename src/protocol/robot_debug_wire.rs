//? Size of message:
//?   - Size of message: 1,638 Bytes
//?   - Size for 12 robots: 1,638 Bytes * 12 = 19536 Bytes => 19,5kB
//?   - Throughput at 100Hz: 19,536 Bytes * 100 = 195,360 Bytes/s => 195kB/s
//?
//? This is all without wrapper message
//? More than suitable for WiFi communication with a channel width of 40MHz

//? This is the message containing
//? all of the onboard sensor data
//?
//? It is sent over wifi, because latency
//? is not that important and more than
//? acceptable when using WiFi 7 6GHz

/// Message from the robot containing debug information
/// todo!("Find more debug data to send back");
#[derive(Debug, Clone, PartialEq)]
pub struct RobotDebugWire {
  pub robot_id: u8,

  pub motor_current: [u8; 5],
  pub motor_rotations: [u16; 5],
  pub motor_encoder_rotations: [u16; 4], // Dribbler has no encoder

  /// Temps
  ///   - 1-5: ESC 1-5
  ///   - 6: Power Board
  ///   - 7: Kicker Board
  ///   - 8: Coil_1
  ///   - 9: Coil_2
  ///   - 10: CM5
  ///   - 11: Control Board
  ///   - 12: SDR
  pub temps: [u8; 12],
}
