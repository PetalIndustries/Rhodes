use crate::{platform::{glfw::PlatformGLFW, PlatformAPI}, primitives::ecs::engine::Engine, systems::{input::InputSystem, render::RenderSystem}};

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

pub fn init() -> anyhow::Result<()> {
  let mut platform = {
    #[cfg(feature = "glfw")]
    {
      Box::new(PlatformGLFW::new())
    }

    #[cfg(feature = "webgl")]
    {
      Box::new(PlatformWebGL::new())
    }
  };

  platform.initialize();

  let engine = Engine::new(platform);
  engine.add_system(InputSystem::new());
  engine.add_system(RenderSystem::new());

  let dt = 1.0 / 60.0;
  for frame in 0 ..5 {
    println!("--- frame {} ---", frame);
    engine.update(dt);
  }

  Ok(())
}