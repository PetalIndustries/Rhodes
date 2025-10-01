use crate::runtimeapi::PlatformAPI;

pub struct PlatformWebGL;

impl PlatformWebGL {
  pub fn new() -> Self {
    Self {}
  }
}

impl PlatformAPI for PlatformWebGL {
  fn destroy(&self) {}
  fn set_title(&self, _title: &str) {}
  fn is_window_closed(&self) {}
  fn is_window_focused(&self) {}
  // fn clear_buffer(col: Pixel)
  fn on_before_draw(&self) {}
  fn on_after_draw(&self) {}
  fn flush_screen(&self, _vsync: bool) {}
  fn poll_events(&self) {}
  fn draw_polygon(&self) {}
}