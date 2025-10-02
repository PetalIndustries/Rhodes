use crate::platformapi::{glfw::PlatformGLFW, PlatformAPI};

#[cfg(test)]
mod tests;

mod core;
mod platformapi;

#[cfg(all(feature = "glfw", feature = "webgl"))]
compile_error!("Features 'glfw' and 'webgl' cannot be enabled at the same time");

#[cfg(not(any(feature = "glfw", feature = "webgl")))]
compile_error!("You must enable exactly one feature: either 'glfw' or 'webgl'");

pub fn init() -> anyhow::Result<()> {
  let platform: Box<dyn PlatformAPI> = {
    #[cfg(feature = "glfw")]
    {
      Box::new(PlatformGLFW::new())
    }

    #[cfg(feature = "webgl")]
    {
      Box::new(PlatformWebGL::new())
    }
  };

  platform.construct_window();

  Ok(())
}