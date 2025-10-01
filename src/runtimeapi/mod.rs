pub mod primitives;
pub(crate) mod runtimes;

pub trait PlatformAPI {
  fn destroy(&self);
  fn set_title(&self, title: &str);
  fn is_window_closed(&self);
  fn is_window_focused(&self);
  // fn clear_buffer(col: Pixel);
  fn on_before_draw(&self);
  fn on_after_draw(&self);
  fn flush_screen(&self, vsync: bool);
  fn poll_events(&self);

  fn draw_polygon(&self);
}