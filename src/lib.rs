use crate::{primitives::ecs::engine::Engine, systems::{input::input_system, render::render_system}};

#[cfg(test)]
mod tests;

mod platform;
mod primitives;
// mod states;
mod systems;

#[cfg(all(feature = "glfw", feature = "webgl"))]
compile_error!("Features 'glfw' and 'webgl' cannot be enabled at the same time");

#[cfg(not(any(feature = "glfw", feature = "webgl")))]
compile_error!("You must enable exactly one feature: either 'glfw' or 'webgl'");

pub fn init() -> Engine {
  let mut engine = Engine::new();
  engine.add_system(input_system);
  engine.add_system(render_system);

  engine
}