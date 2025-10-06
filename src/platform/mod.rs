use crate::core::primitives::color::Color;
use crate::core::primitives::vec2d::Vec2i;
use crate::core::states::input::Key;

pub mod glfw;
pub mod webgl;

pub trait PlatformAPI {
  fn initialize(&mut self);
  fn destroy(&self);
  fn set_title(&mut self, title: &str);
  fn is_window_closed(&self) -> bool;
  fn is_window_focused(&self) -> bool;
  fn clear_buffer(&self, color: Color);
  fn on_before_draw(&self);
  fn on_after_draw(&self);
  fn flush_screen(&self, vsync: bool);
  fn poll_events(&self);

  fn draw_polygon(&self);

  fn construct_window(&mut self, size: &mut Vec2i, is_vsync: bool, is_fullscreen: bool);
}

pub trait IntoRhodesKey {
  fn into(self) -> Key;
}
