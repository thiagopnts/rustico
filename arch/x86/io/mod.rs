use platform::drivers::vga;
use core::option::{Some, None};
use core::{slice, str};
use core::iter::Iterator;
use core::str::StrSlice;

pub fn putc(c: u8) {
    vga::putchar(vga::Char::new_char(c as char));
}

pub fn newline() {
    vga::newline();
}

pub fn write(s: &str) {
  for b in s.bytes() {
    putc(b);
  }
}

