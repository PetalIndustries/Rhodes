use glfw::{WindowHint, WindowMode, Action, MouseButton, Modifiers};
use glfw::{ffi::glfwTerminate, Glfw, PWindow, Error, Window};
use crate::{primitives::color::Color, platform::PlatformAPI};
use crate::primitives::vec2d::Vec2i;
use std::path::PathBuf;

pub struct PlatformGLFW {
  window: Option<PWindow>,
  context: Option<Glfw>,
}

fn error_handler(code: Error, description: String) {
  if code != Error::InvalidEnum {
    log::error!("[GLFW] Runtime error {description}\n\t{}", code.to_string());
    panic!("GLFW Runtime error");
  }
}

macro_rules! load_proc {
  ($w:ident,$p:ident) => {
    gl::$p::load_with(|n| $w.get_proc_address(n).unwrap() as *const _);
  }
}

fn drop_callback(window: &mut Window, paths: Vec<PathBuf>) {
}

fn scroll_callback(window: &mut Window, x: f64, y: f64) {
}

fn cursor_pos_callback(window: &mut Window, x: f64, y: f64) {
}

fn mouse_button_callback(window: &mut Window, button: MouseButton, action: Action, _: Modifiers) {
}

fn key_callback(window: &mut Window, key: glfw::Key, scancode: i32, action: Action, _: Modifiers) {
}

impl PlatformGLFW {
  pub fn new() -> Self {
    Self {
      window: None,
      context: None
    }
  }
}

impl PlatformAPI for PlatformGLFW {
  fn initialize(&mut self) {
    self.context = Some(glfw::init(error_handler).expect("Failed to initialize GLFW"));
  }

  fn destroy(&self) {
    unsafe { glfwTerminate(); }
  }

  fn set_title(&mut self, title: &str) {
    if let Some(window) = &mut self.window {
      window.set_title(title);
    }
  }

  fn is_window_closed(&self) -> bool {
    if let Some(window) = &self.window {
      return window.should_close()
    }

    true
  }

  fn is_window_focused(&self) -> bool {
    if let Some(window) = &self.window {
      return window.is_focused()
    }

    false
  }

  fn clear_buffer(&self, color: Color) {
    unsafe {
      gl::ClearColor(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  fn on_before_draw(&self) {}

  fn on_after_draw(&self) {}

  fn flush_screen(&self, _vsync: bool) {}

  fn poll_events(&self) {}
  fn draw_polygon(&self) {}

  fn construct_window(&mut self, size: &mut Vec2i, is_vsync: bool, is_fullscreen: bool) {
    if let Some(ctx) = &mut self.context {
      ctx.window_hint(WindowHint::DoubleBuffer(is_vsync));

      if ctx.get_platform() == glfw::Platform::MacOS {
        // Disabling window resizing fixes all scaling
        // problems on Apple Retina displays
        ctx.window_hint(WindowHint::Resizable(false));
        ctx.window_hint(WindowHint::CocoaRetinaFramebuffer(false));
      }

      ctx.with_primary_monitor(|ctx, monitor| {
        if let Some(monitor) = monitor {
          let mode = if is_fullscreen { WindowMode::FullScreen(monitor) } else { WindowMode::Windowed };
          let (mut window, _) = ctx.create_window(size.x as u32, size.y as u32, "Rhodes", mode).expect("Failed to initialize GLFW Window");

          ctx.make_context_current(Some(&window));

          window.set_drag_and_drop_callback(drop_callback);
          window.set_scroll_callback(scroll_callback);
          window.set_cursor_pos_callback(cursor_pos_callback);
          window.set_mouse_button_callback(mouse_button_callback);
          window.set_key_callback(key_callback);

          load_proc!(window, ClearColor);
          load_proc!(window, Clear);
          load_proc!(window, Hint);
          load_proc!(window, Enable);

          self.window = Some(window);

          if is_vsync {
            if let Some(video_mode) = monitor.get_video_mode() {
              ctx.set_swap_interval(glfw::SwapInterval::Sync(1));
              ctx.window_hint(WindowHint::RefreshRate(Some(video_mode.refresh_rate)));
            }
          }
        } else {
          log::error!("[GLFW] Can't find primary monitor");
          panic!("GLFW runtime error");
        }
      });

      unsafe { gl::Enable(gl::TEXTURE_2D); }
    }
  }
}