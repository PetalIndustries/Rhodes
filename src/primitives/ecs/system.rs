use crate::primitives::ecs::engine::Engine;

pub trait System {
  fn init(&self, engine: &Engine);

  fn update(&self, engine: &Engine);

  fn destroy(self, engine: &Engine);
}