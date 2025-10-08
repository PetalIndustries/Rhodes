use std::collections::HashMap;
use mlua::Lua;
use crate::{primitives::ecs::{world::World}, platform::PlatformAPI};

pub struct Engine {
  world: World,

  systems: HashMap<usize, SystemClojure>,
  next_system_id: usize,

  platform: Box<dyn PlatformAPI>,
  lua: Lua
}

pub type SystemClojure = Box<dyn FnMut(&mut World)>;

impl Engine {
  pub fn new() -> Self {
    let platform = Self::detect_platform();

    Self {
      world: World::default(),
      systems: HashMap::new(),
      next_system_id: 0,
      lua: Lua::new(),
      platform
    }
  }

  fn detect_platform() -> Box<dyn PlatformAPI> {
    #[cfg(feature = "glfw")]
    {
      Box::new(crate::platform::glfw::PlatformGLFW::new())
    }

    #[cfg(feature = "webgl")]
    {
      Box::new(crate::platform::webgl::PlatformWebGL::new())
    }
  }

  // todo: use delta_time
  pub fn update(&mut self, _dt: f64) {
    for (_, system) in self.systems.iter_mut() {
      system(&mut self.world);
    }
  }

  // systems

  pub fn add_system<F: FnMut(&mut World) + 'static>(&mut self, system: F) -> usize {
    let id = self.next_system_id;
    self.next_system_id += 1;
    self.systems.insert(id, Box::new(system));

    id
  }

  pub fn remove_system(&mut self, index: usize) {
    self.systems.remove(&index);
  }

  // world

  pub fn get_world<'a>(&'a self) -> &'a World {
    &self.world
  }
}