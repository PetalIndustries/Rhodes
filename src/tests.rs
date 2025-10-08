use crate::primitives::typeidmap::TypeIdMap;

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

pub struct SampleState;
#[derive(Debug)]
pub struct InputState<'a> {
  example_string: &'a str
}

#[test]
fn test_typeid_storage() {
  let mut storage = TypeIdMap::new();
  storage.insert(SampleState);

  assert!(storage.get::<SampleState>().is_some(), "SampleState not found in TypeIdMap");
  assert!(storage.get::<InputState>().is_none(), "InputState found in TypeIdMap (is not None)");
}

#[test]
fn test_world_resource() {
  let mut engine = crate::init();

  let world = engine.get_world_mut();
  world.add_resource(InputState { example_string: "example caption" });

  let input = world.get_resource::<InputState>();

  assert!(input.is_some(), "InputState is not within world.resources");

  println!("InputState is {input:?}");
}