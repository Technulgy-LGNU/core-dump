use crate::proto::{Vector2, Vector3};
use num_traits::{NumCast, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
  X,
  Y,
}

impl<T> Vec2<T> {
  #[inline]
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  #[inline]
  pub fn new_from_ssl_vec2(v: Vector2) -> Vec2<T>
  where
    T: NumCast + Copy,
  {
    Vec2 {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
    }
  }

  #[inline]
  pub fn new_from_ssl_vec3(v: Vector3) -> Vec2<T>
  where
    T: NumCast + Copy,
  {
    Vec2 {
      x: NumCast::from(v.x).unwrap(),
      y: NumCast::from(v.y).unwrap(),
    }
  }
}

impl<T: Default> Default for Vec2<T> {
  fn default() -> Self {
    Self {
      x: T::default(),
      y: T::default(),
    }
  }
}

impl<T: Zero> Vec2<T> {
  pub fn zero() -> Self {
    Self {
      x: T::zero(),
      y: T::zero(),
    }
  }
}

impl<T> Vec3<T> {
  #[inline]
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  /// Drops the `z` component, projecting onto the XY plane.
  #[inline]
  pub fn xy(self) -> Vec2<T> {
    Vec2::new(self.x, self.y)
  }
}

impl<T: Default> Default for Vec3<T> {
  fn default() -> Self {
    Self {
      x: T::default(),
      y: T::default(),
      z: T::default(),
    }
  }
}

impl<T: Zero> Vec3<T> {
  pub fn zero() -> Self {
    Self {
      x: T::zero(),
      y: T::zero(),
      z: T::zero(),
    }
  }
}
