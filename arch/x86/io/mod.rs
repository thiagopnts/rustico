use platform::drivers::vga;

use core::option::{Some, None};
use core::{slice, str};
use core::iter::Iterator;

pub fn putc(c: u8) {
  unsafe {
    putchar(vga::curr_x, vga::curr_y, c);
    vga::curr_x += 1;
    if vga::curr_x > vga::WIDTH {
        vga::curr_x -= vga::WIDTH;
        vga::curr_y += 1;
    }
  }
}

pub fn putchar(x: u16, y: u16, c: u8) {
  if x >= vga::WIDTH || y >= vga::HEIGHT {
    return;
  }
  let idx : uint =  (y * vga::WIDTH * 2 + x * 2) as uint;
  unsafe {
    *((vga::ADDRESS + idx) as *mut u16) = vga::make_vgaentry(c, vga::fg_color, vga::bg_color);
  }
}

pub fn newline() {
  unsafe {
    vga::curr_x = 0;
    vga::curr_y += 1;
  }
}

pub fn write(s: &str) {
  for b in slice::iter(str::as_bytes(s)) {
    putc(*b);
  }
}

