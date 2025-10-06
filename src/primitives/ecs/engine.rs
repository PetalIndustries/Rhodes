use crate::{primitives::ecs::{system::System, world::World}, platform::PlatformAPI};

pub struct Engine {
  world: World,
  systems: Vec<Box<dyn System>>,
  platform: Box<dyn PlatformAPI>
}

impl Engine {
  pub fn new(platform: Box<dyn PlatformAPI>) -> Self {
    Self {
      world: World::default(),
      systems: Vec::default(),
      platform
    }
  }

  pub fn update(&self, dt: f64) {
    for system in self.systems.iter() {
      system.update(self);
    }
  }

  // systems

  pub fn attach_system(&mut self, system: Box<dyn System>) {
    self.systems.push(system);
  }

  pub fn detach_system(&mut self, index: usize) {
    self.systems.
  }

  // world

  pub fn get_world<'a>(&'a self) -> &'a World {
    &self.world
  }

  pub fn set_world(&mut self, world: World){
    self.world = world;
  }
}