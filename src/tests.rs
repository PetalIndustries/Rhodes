use crate::primitives::{typeidstorage::TypeIdStorage, vec2d::Vec2f};

// #[test]
// fn math_lerp() -> anyhow::Result<()> {
//   let v: Vec2f = Vec2f::default();

//   println!("Old: {v:?}");

//   v.lerp(&Vec2f::new(1.0, 4.0), 2.0);

//   println!("After lerp: {v:?}");

//   Ok(())
// }

// #[test]
// fn run_glfw() -> anyhow::Result<()> {
//   crate::init()?;

//   Ok(())
// }

#[derive(Debug)]
pub struct AssState(pub i32, pub i32);
#[derive(Debug)]
pub struct InputState(pub i32, pub i32);

#[test]
fn test_typeid_storage() {
  let mut storage = TypeIdStorage::new();
  storage.insert(AssState(0, 3));

  let ass_state = storage.get::<AssState>();

  println!("AssState: {ass_state:?}");

  let input_state = storage.get::<InputState>();

  println!("InputState: {input_state:?}");
}