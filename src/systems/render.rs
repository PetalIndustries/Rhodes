use crate::primitives::ecs::{engine::Engine, system::System};

pub struct RenderSystem;

impl System for RenderSystem {
  fn init(&self, engine: &Engine) { }

  fn update(&self, engine: &Engine) {

  }

  fn destroy(self, engine: &Engine) { }
}