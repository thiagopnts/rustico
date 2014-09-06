
use core::option::*;
use core::mem::transmute;
use core::{str, slice};
use core::iter::range;

pub static ADDRESS: uint = 0xb8000;
pub static WIDTH: u16 = 80;
pub static HEIGHT: u16 = 25;
pub static SCREEN_SIZE: uint = WIDTH as uint * HEIGHT as uint;

pub static mut curr_x: u16 = 0;
pub static mut curr_y: u16 = 0;

pub static DEFAULT_FG: Color = Green;
pub static DEFAULT_BG: Color = Black;

pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

// The screen resolution is 80x25, the root address is 0xb8000
type VGA = [Char, ..SCREEN_SIZE];

struct Display {
  screen: *mut VGA
}

pub static DISPLAY: Display = Display { screen: ADDRESS as *mut VGA };

// One char in the screen is composed by 2 bytes, 1 byte for the character itself
// and another for styling(foreground and background).
pub struct Char {
  pub char: u8,
  style: u8, // 4 bits for foreground and 4 bits for background
}

impl Char {
  pub fn new(c: char, fg: Color, bg: Color) -> Char {
    Char { char: c as u8, style: fg as u8 | (bg as u8 << 4) }
  }
  pub fn new_char(c: char) -> Char {
    Char { char: c as u8, style: DEFAULT_FG as u8 | (DEFAULT_BG as u8 << 4) }
  }
}



pub fn make_vgaentry(c: Char) -> u16 {
  return c.char as u16 | (c.style as u16 << 8);
}

