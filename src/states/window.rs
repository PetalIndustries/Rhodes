use crate::PlatformAPI;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::primitives::vec2d::Vec2i;

pub struct Window {
  platform: Rc<RefCell<dyn PlatformAPI>>,

  title: String,

  window_size: Vec2i,
  is_fullscreen: bool,
  is_vsync: bool,

  dropped_files_cache: Vec<String>
}