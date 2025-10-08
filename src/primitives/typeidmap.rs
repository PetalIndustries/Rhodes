use std::{any::{Any, TypeId}, collections::HashMap};

#[derive(Debug, Default)]
pub struct TypeIdMap {
  storage: HashMap<TypeId, Box<dyn Any>>
}

impl TypeIdMap {
  pub fn new() -> Self {
    Self { storage: HashMap::new() }
  }

  pub fn insert<T: 'static>(&mut self, value: T) {
    self.storage.insert(TypeId::of::<T>(), Box::new(value));
  }

  pub fn get<T: 'static>(&self) -> Option<&T> {
    let key = TypeId::of::<T>();

    self.storage.get(&key)
      .map(|v| v.downcast_ref().expect("failed to downcast Box<T> to T"))
  }

  pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
    let key = TypeId::of::<T>();

    self.storage.get_mut(&key)
      .map(|v| v.downcast_mut().expect("failed to downcast_mut Box<T> to T"))
  }

  pub fn remove<T: 'static>(&mut self) {
    let key = TypeId::of::<T>();

    self.storage.remove(&key);
  }
}