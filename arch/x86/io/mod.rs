use platform::drivers::vga::{ADDRESS, WIDTH, HEIGHT, Char, display};
use core::option::{Some, None};
use core::{slice, str};
use core::iter::Iterator;
use core::str::StrSlice;

pub fn putc(c: u8) {
  unsafe {
    putchar(c);
  }
}

pub fn putchar(c: u8) {
    //FIXME an static mutable struct is unsafe, I have to figure out
    // how to make a proper singleton(or something equivalent)
    unsafe { display.putchar(Char::new_char(c as char)); }
}

pub fn newline() {
  unsafe {
    display.x = 0;
    display.y += 1;
  }
}

pub fn write(s: &str) {
  for b in s.bytes() {
    putc(b);
  }
}

