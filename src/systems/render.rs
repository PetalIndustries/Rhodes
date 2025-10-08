use crate::primitives::ecs::{world::World, system::System};

pub struct RenderSystem;

impl System for RenderSystem {
  fn init(&self, _world: &World) { }

  fn update(&self, _world: &World) { }

  fn destroy(self, _world: &World) { }
}