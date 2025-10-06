use glfw::{WindowHint, WindowMode, Action, MouseButton, Modifiers};
use glfw::{ffi::glfwTerminate, Glfw, PWindow, Error, Window};
use crate::{core::primitives::color::Color, platformapi::{PlatformAPI, IntoRhodesKey}};
use crate::core::primitives::vec2d::Vec2i;
use std::path::PathBuf;
use crate::core::states::input::Key;

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

impl IntoRhodesKey for glfw::Key {
  fn into(self) -> Key {
    match self {
      glfw::Key::Space        => Key::Space,
      glfw::Key::Apostrophe   => Key::Apostrophe,
      glfw::Key::Comma        => Key::Comma,
      glfw::Key::Minus        => Key::Minus,
      glfw::Key::Period       => Key::Period,
      glfw::Key::Slash        => Key::Slash,

      glfw::Key::Num0         => Key::K0,
      glfw::Key::Num1         => Key::K1,
      glfw::Key::Num2         => Key::K2,
      glfw::Key::Num3         => Key::K3,
      glfw::Key::Num4         => Key::K4,
      glfw::Key::Num5         => Key::K5,
      glfw::Key::Num6         => Key::K6,
      glfw::Key::Num7         => Key::K7,
      glfw::Key::Num8         => Key::K8,
      glfw::Key::Num9         => Key::K9,

      glfw::Key::Semicolon    => Key::Semicolon,
      glfw::Key::Equal        => Key::Equal,

      glfw::Key::A            => Key::A,
      glfw::Key::B            => Key::B,
      glfw::Key::C            => Key::C,
      glfw::Key::D            => Key::D,
      glfw::Key::E            => Key::E,
      glfw::Key::F            => Key::F,
      glfw::Key::G            => Key::G,
      glfw::Key::H            => Key::H,
      glfw::Key::I            => Key::I,
      glfw::Key::J            => Key::J,
      glfw::Key::K            => Key::K,
      glfw::Key::L            => Key::L,
      glfw::Key::M            => Key::M,
      glfw::Key::N            => Key::N,
      glfw::Key::O            => Key::O,
      glfw::Key::P            => Key::P,
      glfw::Key::Q            => Key::Q,
      glfw::Key::R            => Key::R,
      glfw::Key::S            => Key::S,
      glfw::Key::T            => Key::T,
      glfw::Key::U            => Key::U,
      glfw::Key::V            => Key::V,
      glfw::Key::W            => Key::W,
      glfw::Key::X            => Key::X,
      glfw::Key::Y            => Key::Y,
      glfw::Key::Z            => Key::Z,

      glfw::Key::LeftBracket  => Key::LeftBracket,
      glfw::Key::Backslash    => Key::Backslash,
      glfw::Key::RightBracket => Key::RightBracket,

      glfw::Key::Escape       => Key::Escape,
      glfw::Key::Enter        => Key::Enter,
      glfw::Key::Tab          => Key::Tab,
      glfw::Key::Backspace    => Key::Backspace,
      glfw::Key::Insert       => Key::Insert,
      glfw::Key::Delete       => Key::Delete,
      glfw::Key::Right        => Key::Right,
      glfw::Key::Left         => Key::Left,
      glfw::Key::Down         => Key::Down,
      glfw::Key::Up           => Key::Up,
      glfw::Key::PageUp       => Key::PageUp,
      glfw::Key::PageDown     => Key::PageDown,
      glfw::Key::Home         => Key::Home,
      glfw::Key::End          => Key::End,

      glfw::Key::CapsLock     => Key::CapsLock,
      glfw::Key::ScrollLock   => Key::ScrollLock,
      glfw::Key::NumLock      => Key::NumLock,
      glfw::Key::PrintScreen  => Key::PrintScreen,
      glfw::Key::Pause        => Key::Pause,

      glfw::Key::F1           => Key::F1,
      glfw::Key::F2           => Key::F2,
      glfw::Key::F3           => Key::F3,
      glfw::Key::F4           => Key::F4,
      glfw::Key::F5           => Key::F5,
      glfw::Key::F6           => Key::F6,
      glfw::Key::F7           => Key::F7,
      glfw::Key::F8           => Key::F8,
      glfw::Key::F9           => Key::F9,
      glfw::Key::F10          => Key::F10,
      glfw::Key::F11          => Key::F11,
      glfw::Key::F12          => Key::F12,
      glfw::Key::F13          => Key::F13,
      glfw::Key::F14          => Key::F14,
      glfw::Key::F15          => Key::F15,
      glfw::Key::F16          => Key::F16,
      glfw::Key::F17          => Key::F17,
      glfw::Key::F18          => Key::F18,
      glfw::Key::F19          => Key::F19,
      glfw::Key::F20          => Key::F20,
      glfw::Key::F21          => Key::F21,
      glfw::Key::F22          => Key::F22,
      glfw::Key::F23          => Key::F23,
      glfw::Key::F24          => Key::F24,

      glfw::Key::Kp0          => Key::Np0,
      glfw::Key::Kp1          => Key::Np1,
      glfw::Key::Kp2          => Key::Np2,
      glfw::Key::Kp3          => Key::Np3,
      glfw::Key::Kp4          => Key::Np4,
      glfw::Key::Kp5          => Key::Np5,
      glfw::Key::Kp6          => Key::Np6,
      glfw::Key::Kp7          => Key::Np7,
      glfw::Key::Kp8          => Key::Np8,
      glfw::Key::Kp9          => Key::Np9,
      glfw::Key::KpDecimal    => Key::NpDecimal,
      glfw::Key::KpDivide     => Key::NpDivide,
      glfw::Key::KpMultiply   => Key::NpMultiply,
      glfw::Key::KpSubtract   => Key::NpSubtract,
      glfw::Key::KpAdd        => Key::NpAdd,
      glfw::Key::KpEnter      => Key::NpEnter,
      glfw::Key::KpEqual      => Key::NpEqual,

      glfw::Key::LeftShift    => Key::LeftShift,
      glfw::Key::LeftControl  => Key::LeftControl,
      glfw::Key::LeftAlt      => Key::LeftAlt,
      glfw::Key::LeftSuper    => Key::LeftSuper,
      glfw::Key::RightShift   => Key::RightShift,
      glfw::Key::RightControl => Key::RightControl,
      glfw::Key::RightAlt     => Key::RightAlt,
      glfw::Key::RightSuper   => Key::RightSuper,
      glfw::Key::Menu         => Key::Menu,

      _ => Key::None
    }
  }
}

impl PlatformAPI for PlatformGLFW {
  fn initialize(&mut self) {
    self.context = Some(glfw::init(error_handler).expect("Failed to initialize GLFW"));

    if let Some(window) = &mut self.window {
      load_proc!(window, ClearColor);
      load_proc!(window, Clear);
      load_proc!(window, Hint);
      load_proc!(window, Enable);
    }
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