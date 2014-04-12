#![crate_id = "rustic#0.0"]
#![allow(ctypes)]
#![no_std]

extern crate core;

use core::str::as_bytes;

static VGA_ADDRESS: uint = 0xb8000;
static VGA_WIDTH: u16 = 80;
//static VGA_HEIGHT: u16 = 80;

static mut fg_color: Color = Green;
static mut bg_color: Color = Black;

enum Color {
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

pub fn putchar(x: u16, y: u16, c: u8) {
  let idx: uint = (y * VGA_WIDTH * 2 + x * 2) as uint;
  unsafe {
    *((VGA_ADDRESS + idx) as *mut u16) = make_vgaentry(c, fg_color, bg_color);
  }
}


unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}

#[no_mangle]
pub unsafe fn main() {
    clear_screen(Black);
    putchar(0, 0, 79);
    putchar(1, 0, 108);
    putchar(2, 0, 97);
    putchar(3, 0, 114);
    putchar(4, 0, 32);
    putchar(5, 0, 109);
    putchar(6, 0, 117);
    putchar(7, 0, 110);
    putchar(8, 0, 100);
    putchar(9, 0, 111);
}
