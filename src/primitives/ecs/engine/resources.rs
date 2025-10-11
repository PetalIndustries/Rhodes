#[derive(Default)]
pub struct InputResource {
  mouse: MouseData
}

#[derive(Default)]
pub struct MouseData(pub i32, pub i32);

#[derive(Default)]
pub struct TimeResource {
  delta_time: f32
}

impl TimeResource {
  pub(super) fn update_delta_time(&mut self, delta_time: f32) {
    self.delta_time = delta_time;
  }

  pub fn delta_time(&self) -> f32 {
    self.delta_time
  }
}