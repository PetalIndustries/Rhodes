use std::{any::Any, collections::HashMap};
use crate::primitives::{ecs::entity::Entity, typeidstorage::TypeIdStorage};

pub struct World {
  next_id: u32,
  entities: HashMap<u32, Entity>,
  // 0      Entity(0)
  // 1      Entity(1)
  // 2      Entity(2)
  // 3      Entity(3)
  components: TypeIdStorage<HashMap<Entity, Box<dyn Any>>>, //HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>       value -> HashMap<Entity, Box<dyn Any>>,
  // global game state
  // resources: TypeIdStorage<Box<dyn Any>>
  resources: TypeIdStorage
}

impl Default for World {
  fn default() -> Self {
    Self {
      next_id: 0,
      entities: HashMap::new(),
      components: TypeIdStorage::new(),
      resources: TypeIdStorage::new(),
    }
  }
}

impl World {
  // todo: revisiion return value
  // probably user won't use Entity type
  pub fn create_entity(&mut self) -> Entity {
    let id = self.next_id;
    self.next_id += 1;

    let entity = Entity(id);
    self.entities.insert(id, entity.clone());

    entity
  }

  pub fn add_resource<T: 'static>(&mut self, value: T) {
    self.resources.insert(Box::new(value));
  }

  pub fn remove_resource<T: 'static>(&mut self) {
    self.resources.remove::<T>()
  }
}