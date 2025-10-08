use crate::primitives::ecs::{engine::Engine, world::World};

#[cfg(test)]
mod tests;

mod platform;
mod primitives;
mod states;
mod systems;

#[cfg(all(feature = "glfw", feature = "webgl"))]
compile_error!("Features 'glfw' and 'webgl' cannot be enabled at the same time");

#[cfg(not(any(feature = "glfw", feature = "webgl")))]
compile_error!("You must enable exactly one feature: either 'glfw' or 'webgl'");

fn input_system(world: &mut World) { }
fn render_system(world: &mut World) { }

pub fn init() {
  let mut engine = Engine::new();
  engine.add_system(input_system);
  engine.add_system(render_system);

  // let dt = 1.0 / 60.0;
  // for frame in 0 ..5 {
  //   println!("--- frame {} ---", frame);
  //   engine.update(dt);
  // }

  // engine.handle()
}