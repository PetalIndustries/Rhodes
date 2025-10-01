use crate::runtimeapi::{runtimes::{glfw::PlatformGLFW, webgl::PlatformWebGL}, PlatformAPI};

#[cfg(test)]
mod tests;
mod runtimeapi;

pub enum PlatformKind {
  GLFW,
  WebGL
}

pub fn init(platform: PlatformKind) -> anyhow::Result<()> {
  let platform: Box<dyn PlatformAPI> = match platform {
    PlatformKind::GLFW => Box::new(PlatformGLFW::new()),
    PlatformKind::WebGL => Box::new(PlatformWebGL::new())
  };

  platform.destroy();

  Ok(())
}