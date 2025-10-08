use crate::primitives::ecs::world::World;
// use crate::states::input::Input;
use crate::primitives::ecs::system::System;

pub struct InputSystem;

impl System for InputSystem {
  fn init(&self, world: &World) { }

  fn update(&self, world: &World) { }

  fn destroy(self, world: &World) { }
}