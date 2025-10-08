use crate::primitives::ecs::world::World;

pub trait System {
  fn init(&self, world: &World);

  fn update(&self, world: &World);

  fn destroy(self, world: &World);
}