use std::{any::{type_name, Any, TypeId}, collections::HashMap};

#[derive(Debug)]
pub struct TypeIdStorage<T = Box<dyn Any>>
where
  T: 'static
{
  storage: HashMap<TypeId, T>
}

impl<T> Default for TypeIdStorage<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: 'static> TypeIdStorage<T> {
  pub fn new() -> Self {
    Self { storage: HashMap::new() }
  }

  pub fn insert(&mut self, value: T) {
    self.storage.insert(TypeId::of::<T>(), value);
  }

  pub fn get<>(&self) -> &T {
    let key = TypeId::of::<T>();

    if let Some(value) = self.storage.get(&key) {
      return value
    }

    panic!("no {:?}", type_name::<T>())
  }

  pub fn get_mut(&mut self) -> &mut T {
    let key = TypeId::of::<T>();

    if let Some(value) = self.storage.get_mut(&key) {
      return value
    }

    panic!("no {:?}", type_name::<T>())
  }

  pub fn remove<T>(&mut self) {
    let key = TypeId::of::<T>();

    self.storage.remove(&key);
  }

  // pub fn print_all_types(&self) {
  //   for key in self.storage.keys() {
  //     println!("Stored TypeId: {:?}", key);
  //   }
  // }
}