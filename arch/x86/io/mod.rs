use platform::drivers::vga::{ADDRESS, WIDTH, HEIGHT, Char, make_vgaentry, curr_x, curr_y};
use core::option::{Some, None};
use core::{slice, str};
use core::iter::Iterator;
use core::str::StrSlice;

pub fn putc(c: u8) {
  unsafe {
    putchar(curr_x, curr_y, c);
    curr_x += 1;
    if curr_x > WIDTH {
        curr_x -= WIDTH;
        curr_y += 1;
    }
  }
}

pub fn putchar(x: u16, y: u16, c: u8) {
  if x >= WIDTH || y >= HEIGHT {
    return;
  }
  let idx : uint =  (y * WIDTH * 2 + x * 2) as uint;
  unsafe {
    *((ADDRESS + idx) as *mut u16) = make_vgaentry(Char::new_char(c as char));
  }
}

pub fn newline() {
  unsafe {
    curr_x = 0;
    curr_y += 1;
  }
}

pub fn write(s: &str) {
  for b in s.bytes() {
    putc(b);
  }
}

