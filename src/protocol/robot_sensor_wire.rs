// ? Size of message:
//?   - Size of message: 14,529 Bytes
//?   - Throughput at 500Hz: 14,529 Bytes * 500 = 7.26MB/s
//?   - Throughput at 1000Hz: 14,529 Bytes * 1000 = 14.529MB/s
//?
//? This is all without wrapper message
//? Should be more than suitable for WiFi communication with a channel width of 40MHz

//? This is the message containing
//? all of the onboard sensor data
//?
//? It is sent over wifi, because latency
//? is not that important and more than
//? acceptable when using WiFi 7 6GHz

/// Message containing all the onboard Sensor data:
///   - Lidar
///   - Vision
#[derive(Debug, Clone, PartialEq)]
pub struct RobotSensorWire {
  pub robot_id: u8,
  pub seq: u32,

  // Vision
  pub ball_x: i32,
  pub ball_y: i32,
  pub ball_size: f32,

  // Lidar
  /// 0 to 360 degrees, each degree one distance measurement in mm
  /// Resolution is 0.8 degrees => 450 Measurements
  pub lidar_dist: [u32; 450], // 14.4 kB
}
