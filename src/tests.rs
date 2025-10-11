use crate::primitives::{ecs::world::World, typeidmap::TypeIdMap, vec2d::Vec2f};

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

struct PlayerData {
  pos: Vec2f,
  vel: Vec2f
}

const INVENTORY_SIZE: usize = 32;

struct Inventory {
  content: Vec<i32>
}

#[test]
fn test_ecs() {
  let inventory = Inventory {
    content: Vec::with_capacity(INVENTORY_SIZE)
  };

  let mut world = World::default();

  let mut entity = world.create_entity();
  let mut inventory = world.add_component::<Inventory>(&mut entity);
  let mut player_data = world.add_component::<PlayerData>(&mut entity);

  world.filter::<Inventory, PlayerData>()
}
