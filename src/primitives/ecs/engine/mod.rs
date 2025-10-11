use std::collections::HashMap;
use mlua::Lua;
use crate::{platform::PlatformAPI, primitives::{ecs::{engine::resources::TimeResource, world::World}, vec2d::Vec2i}};
use std::time::Instant;

pub mod resources;

pub type SystemClojure = Box<dyn FnMut(&mut World)>;

pub struct Engine {
  platform: Box<dyn PlatformAPI>,
  lua: Lua,

  world: World,

  systems: HashMap<usize, SystemClojure>,
  next_system_id: usize,

  is_running: bool
}

impl Engine {
  pub fn new() -> Self {
    let mut platform = Self::detect_platform();

    // temporary code
    platform.initialize();
    platform.construct_window(&mut Vec2i::new(800, 600), false, false);

    let mut world = World::default();
    world.add_resource(TimeResource::default());

    Self {
      world,
      systems: HashMap::new(),
      next_system_id: 0,
      lua: Lua::new(),
      platform,
      is_running: true
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

  pub fn start(&mut self) {
    let mut last_update = Instant::now();

    while self.is_running {
      let delta_time = last_update.elapsed().as_secs_f32();
      last_update = Instant::now();

      self.update(delta_time);
    }
  }

  /// Calls all systems
  pub fn update(&mut self, delta_time: f32) {
    let time = self.world.resource_mut::<TimeResource>();
    time.update_delta_time(delta_time);

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

  pub fn get_world_mut<'a>(&'a mut self) -> &'a mut World {
    &mut self.world
  }
}