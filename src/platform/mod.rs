use crate::primitives::color::Color;
use crate::primitives::vec2d::Vec2i;

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