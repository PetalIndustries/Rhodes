use std::panic;
use std::rc::Rc;
use std::cell::RefCell;

use crate::core::primitives::vec2d::Vec2i;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
  None,

  Space,
  Apostrophe,
  Comma,
  Minus,
  Period,
  Slash,

  K0, K1, K2, K3, K4, K5, K6, K7, K8, K9,

  Semicolon,
  Equal,

  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,
  T, U, V, W, X, Y, Z,

  LeftBracket,
  Backslash,
  RightBracket,

  Escape,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  Right,
  Left,
  Down,
  Up,
  PageUp,
  PageDown,
  Home,
  End,

  CapsLock,
  ScrollLock,
  NumLock,
  PrintScreen,
  Pause,

  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
  F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,

  Np0, Np1, Np2, Np3, Np4, Np5, Np6, Np7, Np8, Np9,
  NpDecimal,
  NpDivide,
  NpMultiply,
  NpSubtract,
  NpAdd,
  NpEnter,
  NpEqual,

  LeftShift,
  LeftControl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightControl,
  RightAlt,
  RightSuper,
  Menu,

  Count
}

const KEYS_COUNT: usize = Key::Count as usize;

impl Into<usize> for Key {
  fn into(self) -> usize {
    self as usize
  }
}

impl Key {
  fn to_char_pair(self) -> (char, char) {
    match self {
      Key::Comma => (',', '<'),
      Key::Minus => ('-', '_'),
      Key::Period => ('.', '>'),
      Key::Slash => ('/', '?'),
      Key::K0 => ('0', ')'),
      Key::K1 => ('1', '!'),
      Key::K2 => ('2', '@'),
      Key::K3 => ('3', '#'),
      Key::K4 => ('4', '$'),
      Key::K5 => ('5', '%'),
      Key::K6 => ('6', '^'),
      Key::K7 => ('7', '&'),
      Key::K8 => ('8', '*'),
      Key::K9 => ('9', '('),
      Key::Semicolon => (';', ':'),
      Key::Equal => ('=', '+'),
      Key::A => ('a', 'A'),
      Key::B => ('b', 'B'),
      Key::C => ('c', 'C'),
      Key::D => ('d', 'D'),
      Key::E => ('e', 'E'),
      Key::F => ('f', 'F'),
      Key::G => ('g', 'G'),
      Key::H => ('h', 'H'),
      Key::I => ('i', 'I'),
      Key::J => ('j', 'J'),
      Key::K => ('k', 'K'),
      Key::L => ('l', 'L'),
      Key::M => ('m', 'M'),
      Key::N => ('n', 'N'),
      Key::O => ('o', 'O'),
      Key::P => ('p', 'P'),
      Key::Q => ('q', 'Q'),
      Key::R => ('r', 'R'),
      Key::S => ('s', 'S'),
      Key::T => ('t', 'T'),
      Key::U => ('u', 'U'),
      Key::V => ('v', 'V'),
      Key::W => ('w', 'W'),
      Key::X => ('x', 'X'),
      Key::Y => ('y', 'Y'),
      Key::Z => ('z', 'Z'),
      Key::LeftBracket => ('[', '{'),
      Key::Backslash => ('\\', '|'),
      Key::RightBracket => (']', '}'),
      Key::Np0 => ('0', '0'),
      Key::Np1 => ('1', '1'),
      Key::Np2 => ('2', '2'),
      Key::Np3 => ('3', '3'),
      Key::Np4 => ('4', '4'),
      Key::Np5 => ('5', '5'),
      Key::Np6 => ('6', '6'),
      Key::Np7 => ('7', '7'),
      Key::Np8 => ('8', '8'),
      Key::Np9 => ('9', '9'),
      Key::NpDivide => ('/', '/'),
      Key::NpMultiply => ('*', '*'),
      Key::NpSubtract => ('-', '-'),
      Key::NpAdd => ('+', '+'),
      Key::NpEqual => ('=', '+'),
      _ => panic!("[Input] Not a char: Key::{:?}", self)
    }
  }
}

pub enum Button {
  Left, Right, Wheel,
  Mouse4, Mouse5, Mouse6,
  Mouse7, Mouse8, Count
}

const BUTTONS_COUNT: usize = Button::Count as usize;

impl Into<usize> for Button {
  fn into(self) -> usize {
    self as usize
  }
}

#[derive(Default, Copy, Clone)]
pub struct KeyState {
  held: bool,
  released: bool,
  pressed: bool
}

impl KeyState {
  pub fn new(held: bool, released: bool, pressed: bool) -> Self {
    Self {
      held: held,
      released: released,
      pressed: pressed
    }
  }
}

pub struct Input {
  keys: [KeyState; KEYS_COUNT],
  buttons: [KeyState; BUTTONS_COUNT],

  keys_old_state: [bool; KEYS_COUNT],
  keys_new_state: [bool; KEYS_COUNT],

  buttons_old_state: [bool; BUTTONS_COUNT],
  buttons_new_state: [bool; BUTTONS_COUNT],

  mouse_pos: Vec2i,

  scroll_delta: i32
}

fn flush_buffer(data: &mut [KeyState], new_state: &mut [bool], old_state: &mut [bool]) {
  for i in 0..data.len() {
    data[i].pressed = false;
    data[i].released = false;

    if new_state[i] != old_state[i] {
      if new_state[i] {
        data[i].pressed = !data[i].held;
        data[i].held = true;
      } else {
        data[i].released = true;
        data[i].held = false;
      }
    }

    old_state[i] = new_state[i];
  }
}

impl Input {
  pub fn new() -> Self {
    Self {
      keys: [KeyState::new(false, false, false); KEYS_COUNT],
      buttons: [KeyState::new(false, false, false); BUTTONS_COUNT],

      keys_old_state: [false; KEYS_COUNT],
      keys_new_state: [false; KEYS_COUNT],

      buttons_old_state: [false; BUTTONS_COUNT],
      buttons_new_state: [false; BUTTONS_COUNT],

      mouse_pos: Vec2i::new(0, 0),
      scroll_delta: 0,
    }
  }

  pub fn flush_buffers(&mut self) {
    flush_buffer(&mut self.keys, &mut self.keys_new_state, &mut self.keys_old_state);
    flush_buffer(&mut self.buttons, &mut self.buttons_new_state, &mut self.buttons_old_state);
  }

  pub fn get_key_state<'a>(&'a self, key: Key) -> &'a KeyState {
    &self.keys[key as usize]
  }

  pub fn get_button_state<'a>(&'a self, button: Button) -> &'a KeyState {
    &self.buttons[button as usize]
  }
}
