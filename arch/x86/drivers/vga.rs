use core::option::{Some, None};
use core::{str, slice};
use core::iter::Iterator;

pub static ADDRESS: uint = 0xb8000;
pub static WIDTH: u16 = 80;
pub static HEIGHT: u16 = 80;

pub static mut curr_x: u16 = 0;
pub static mut curr_y: u16 = 0;

pub static mut fg_color: Color = Green;
pub static mut bg_color: Color = Black;

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

fn range(lo: uint, hi: uint, it: |uint| -> ()) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

pub fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
  let color = fg as u16 | (bg as u16 << 4);
  return c as u16 | (color << 8);
}

pub unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

