use core::option::{Some, None};
use core::{str, slice};
use core::iter::Iterator;

static VGA_ADDRESS: uint = 0xb8000;
static VGA_WIDTH: u16 = 80;
static VGA_HEIGHT: u16 = 80;

static mut curr_x: u16 = 0;
static mut curr_y: u16 = 0;

static mut fg_color: Color = Green;
static mut bg_color: Color = Black;

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

fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
  let color = fg as u16 | (bg as u16 << 4);
  return c as u16 | (color << 8);
}

pub unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

pub fn putc(c: u8) {
  unsafe {
    putchar(curr_x, curr_y, c);
    curr_x += 1;
    if curr_x > VGA_WIDTH {
        curr_x -= VGA_WIDTH;
        curr_y += 1;
    }
  }
}

pub fn putchar(x: u16, y: u16, c: u8) {
  if x >= VGA_WIDTH || y >= VGA_HEIGHT {
    return;
  }
  let idx : uint =  (y * VGA_WIDTH * 2 + x * 2) as uint;
  unsafe {
    *((VGA_ADDRESS + idx) as *mut u16) = make_vgaentry(c, fg_color, bg_color);
  }
}

pub fn newline() {
  unsafe {
    curr_x = 0;
    curr_y += 1;
  }
}

pub fn write(s: &str) {
  for b in slice::iter(str::as_bytes(s)) {
    putc(*b);
  }
}
