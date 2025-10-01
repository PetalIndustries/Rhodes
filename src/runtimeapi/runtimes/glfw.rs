use glfw::{Context, PWindow, Key, Action};
use crate::runtimeapi::PlatformAPI;

pub struct PlatformGLFW {
  window: PWindow
}

// UNDER HEAVY CONSTRUCTION!!!

fn error_handler(err: glfw::Error, description: String) {
  println!("GLFW error {:?}: {:?}", err, description);
}

impl PlatformGLFW {
  // just example
  pub fn new() -> Self {
    let mut glfw = glfw::init(error_handler).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "Example GLFW Window", glfw::WindowMode::Windowed)
      .unwrap();

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
      glfw.poll_events();

      for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
              window.set_should_close(true)
            }
            _ => {}
          }
      }
    }

    Self { window }
  }
}

impl PlatformAPI for PlatformGLFW {
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