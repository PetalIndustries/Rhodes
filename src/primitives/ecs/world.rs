use std::{any::{Any, TypeId}, collections::HashMap};
use crate::primitives::ecs::entity::Entity;

#[derive(Default)]
pub struct World {
  next_id: u32,
  entities: HashMap<u32, Entity>,
  // 0      Entity(0)
  // 1      Entity(1)
  // 2      Entity(2)
  // 3      Entity(3)
  components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>
}

impl World {
  pub fn create_entity(&mut self) -> Entity {
    let id = self.next_id;
    self.next_id += 1;
    Entity(id)
  }
}