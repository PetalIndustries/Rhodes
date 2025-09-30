use mlua::{Lua, Result};

pub fn apply_std(lua: &Lua) -> Result<()> {
  let globals = lua.globals();
  globals.set("test_std", env!("CARGO_PKG_VERSION"))?;

  Ok(())
}