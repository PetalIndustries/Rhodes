use std::collections::HashMap;
use crate::primitives::{ecs::entity::Entity, typeidmap::TypeIdMap};

#[derive(Default)]
pub struct World {
  next_id: u32,
  entities: HashMap<u32, Entity>,
  components: TypeIdMap, //HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>       value -> HashMap<Entity, Box<dyn Any>>,
  // global game state
  resources: TypeIdMap
}

impl World {
  // todo: revision return value
  // probably user won't use Entity type
  pub fn create_entity(&mut self) -> Entity {
    let id = self.next_id;
    self.next_id += 1;

    let entity = Entity(id);
    self.entities.insert(id, entity.clone());

    entity
  }

  pub fn add_resource<T: 'static>(&mut self, value: T) {
    self.resources.insert(value);
  }

  pub fn get_resource<'a, T: 'static>(&'a self) -> Option<&'a T> {
    self.resources.get::<T>()
  }

  pub fn get_resource_mut<'a, T: 'static>(&'a mut self) -> Option<&'a mut T> {
    self.resources.get_mut::<T>()
  }

  pub(crate) fn resource<'a, T: 'static>(&'a mut self) -> &T {
    self.resources.get::<T>().unwrap()
  }

  pub(crate) fn resource_mut<'a, T: 'static>(&'a mut self) -> &'a mut T {
    self.resources.get_mut::<T>().unwrap()
  }

  pub fn remove_resource<T: 'static>(&mut self) {
    self.resources.remove::<T>()
  }
}