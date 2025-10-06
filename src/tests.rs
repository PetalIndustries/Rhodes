use crate::primitives::vec2d::Vec2f;

#[test]
fn math_lerp() -> anyhow::Result<()> {
  let v: Vec2f = Vec2f::default();

  println!("Old: {v:?}");

  v.lerp(&Vec2f::new(1.0, 4.0), 2.0);

  println!("After lerp: {v:?}");

  Ok(())
}

#[test]
fn run_glfw() -> anyhow::Result<()> {
  crate::init()?;

  Ok(())
}