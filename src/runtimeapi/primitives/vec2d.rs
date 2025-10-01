use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};
use num_traits::Float;

/// Vector 2D
///
/// Structure that stores x, y coordinates
#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2D<T>
where
  T: Float + Default + Clone + Copy
{
  pub x: T,
  pub y: T,
}

pub type Vec2i = Vec2D<i32>;
pub type Vec2f = Vec2D<f32>;

impl<T: Float + Default + Copy> Vec2D<T> {
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  pub fn clamp(&self, start: &Self, end: &Self) -> Self {
    Self::new(self.x.clamp(start.x, end.x), self.y.clamp(start.y, end.y))
  }

  pub fn lerp(&self, end: &Self, time: T) -> Self {
    let one = T::one();

    Self {
      x: self.x * (one - time) + end.x * time,
      y: self.y * (one - time) + end.y * time,
    }
  }

  pub fn distance(&self, end: &Self) -> T {
    (*self - *end).length()
  }

  pub fn manhattan_distance(&self, end: &Self) -> T {
    (self.x - end.x).abs() + (self.y - end.y).abs()
  }

  pub fn dot_product(&self, other: &Self) -> T {
    self.x * other.x + self.y * other.y
  }

  pub fn cross_product(&self, other: &Self) -> T {
    self.x * other.y - self.y * other.x
  }

  pub fn angle(&self, other: &Self) -> T {
    self.dot_product(other) / (self.length() + other.length())
  }

  pub fn length(&self) -> T {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  pub fn min(&self, other: &Self) -> Self {
    Self::new(self.x.min(other.x), self.y.min(other.y))
  }

  pub fn max(&self, other: &Self) -> Self {
    Self::new(self.x.max(other.x), self.y.max(other.y))
  }

  pub fn swap(&mut self, other: &mut Self) {
    let temp = self.clone();

    self.x = other.x;
    self.y = other.y;

    other.x = temp.x;
    other.y = temp.y;
  }

  pub fn normalise(&self) -> Self {
    let inv = T::one() / self.length();
    Self::new(self.x * inv, self.y * inv)
  }

  pub fn abs(&self) -> Self {
    Self::new(self.x.abs(), self.y.abs())
  }

  pub fn perpendicular(&self) -> Self {
    Self::new(-self.y, self.x)
  }

  pub fn floor(&self) -> Self {
    Self::new(self.x.floor(), self.y.floor())
  }

  pub fn ceil(&self) -> Self {
    Self::new(self.x.ceil(), self.y.ceil())
  }

  pub fn round(&self) -> Self {
    Self::new(self.x.round(), self.y.round())
  }

  pub fn as_cartesian(&self) -> Self {
    Self::new(self.y.cos() * self.x, self.y.sin() * self.y)
  }

  pub fn as_polar(&self) -> Self {
    Self::new(self.length(), self.y.atan2(self.x))
  }
}

macro_rules! impl_calc_trait {
  ($v:ident, $fn:ident, $o:tt) => {
    impl<T: Float + Default + Copy> $v for Vec2D<T> {
      type Output = Self;

      fn $fn(self, rhs: Self) -> Self::Output {
        Self::new(self.x $o rhs.x, self.y $o rhs.y)
      }
    }
  }
}

impl_calc_trait!(Add, add, +);
impl_calc_trait!(Sub, sub, -);
impl_calc_trait!(Mul, mul, *);
impl_calc_trait!(Div, div, /);

macro_rules! impl_calc_assign_trait {
  ($v:ident, $fn:ident, $o:tt) => {
    impl<T: Float + Default + Copy> $v for Vec2D<T> {
      fn $fn(&mut self, rhs: Self) {
        self.x = self.x $o rhs.x;
        self.y = self.y $o rhs.y;
      }
    }
  }
}

impl_calc_assign_trait!(AddAssign, add_assign, +);
impl_calc_assign_trait!(SubAssign, sub_assign, -);
impl_calc_assign_trait!(MulAssign, mul_assign, *);
impl_calc_assign_trait!(DivAssign, div_assign, /);

impl<T: Float + Display + Default> ToString for Vec2D<T> {
  fn to_string(&self) -> String {
    format!("({},{})", self.x, self.y)
  }
}