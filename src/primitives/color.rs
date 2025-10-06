use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug, Eq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

fn f32_to_u8(f: f32) -> u8 {
  (f.clamp(0.0, 1.0) * 255.0).round() as u8
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub fn from_u32(n: u32) -> Self {
    Self {
      r: ((n >> 24) & 0xFF) as u8,
      g: ((n >> 16) & 0xFF) as u8,
      b: ((n >> 8) & 0xFF) as u8,
      a: (n & 0xFF) as u8,
    }
  }

  pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self {
      r: f32_to_u8(r),
      g: f32_to_u8(g),
      b: f32_to_u8(b),
      a: f32_to_u8(a),
    }
  }
}

macro_rules! impl_calc_trait_for_color {
  ($trait:ident, $func:ident, $op:tt) => {
    impl $trait for Color {
      type Output = Self;

      fn $func(self, rhs: Self) -> Self::Output {
        Color {
          r: (self.r as i16 $op rhs.r as i16).clamp(0, 255) as u8,
          g: (self.g as i16 $op rhs.g as i16).clamp(0, 255) as u8,
          b: (self.b as i16 $op rhs.b as i16).clamp(0, 255) as u8,
          a: (self.a as i16 $op rhs.a as i16).clamp(0, 255) as u8,
        }
      }
    }
  };
}

impl_calc_trait_for_color!(Add, add, +);
impl_calc_trait_for_color!(Sub, sub, -);
impl_calc_trait_for_color!(Mul, mul, *);
impl_calc_trait_for_color!(Div, div, /);

macro_rules! impl_calc_assign_trait_for_color {
  ($trait:ident, $func:ident, $op:tt) => {
    impl $trait for Color {
      fn $func(&mut self, rhs: Self) {
        self.r = (self.r as i16 $op rhs.r as i16).clamp(0, 255) as u8;
        self.g = (self.g as i16 $op rhs.g as i16).clamp(0, 255) as u8;
        self.b = (self.b as i16 $op rhs.b as i16).clamp(0, 255) as u8;
        self.a = (self.a as i16 $op rhs.a as i16).clamp(0, 255) as u8;
      }
    }
  };
}

impl_calc_assign_trait_for_color!(AddAssign, add_assign, +);
impl_calc_assign_trait_for_color!(SubAssign, sub_assign, -);
impl_calc_assign_trait_for_color!(MulAssign, mul_assign, *);
impl_calc_assign_trait_for_color!(DivAssign, div_assign, /);

impl Into<u32> for Color {
  fn into(self) -> u32 {
    ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
  }
}

impl ToString for Color {
  fn to_string(&self) -> String {
    format!("({},{},{},{})", self.r, self.g, self.b, self.a)
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
  }
}
