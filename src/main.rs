mod lua;

use anyhow::anyhow;
use glfw::{Action, Context, Key};

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
  match event {
    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
      window.set_should_close(true)
    }
    _ => {}
  }
}

fn main() -> anyhow::Result<()> {
  let mut glfw = glfw::init(glfw::fail_on_errors)?;

  let lua_state = mlua::Lua::new();

  // let rhodes = rhodes::Rhodes::new();
  // std::lua::apply_std(&rhodes.lua)?;
  // rhodes.init();

  /*rhodes.input.get_key(Key::Escape);
  rhodes.draw.rect(0, 0, 800, 600);
  rhodes.audio.play("assets/audio/music.mp3");*/

  lua::std::apply_std(&lua_state)
    .map_err(|e| anyhow!(e.to_string()))?;

  let (mut window, events) = glfw.create_window(800, 600, "Rhodes Game Engine", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();

  while !window.should_close() {
    glfw.poll_events();

    for (_, event) in glfw::flush_messages(&events) {
        handle_window_event(&mut window, event);
    }
  }

  Ok(())
}