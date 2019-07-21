pub struct Vec2<T>(T, T);

use std::ops;

impl ops::Add for Vec2<i32> {
  type Output = Vec2<i32>;

  fn add(self, v: Vec2<i32>) -> Vec2<i32> {
    Vec2(self.0 + v.0, self.0 + v.1)
  }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Vec2<i32> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "( x = {}, y = {} )", self.0, self.1)
  }
}

impl Vec2<i32> {
  pub fn new(x: i32, y: i32) -> Vec2<i32> {
    Vec2(x, y)
  }
}
