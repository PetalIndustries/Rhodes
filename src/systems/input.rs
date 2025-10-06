use crate::primitives::ecs::{engine::Engine, system::System};

pub struct InputSystem;

impl System for InputSystem {
  fn init(&self, engine: &Engine) { }

  fn update(&self, engine: &Engine) {

  }

  fn destroy(self, engine: &Engine) { }
}