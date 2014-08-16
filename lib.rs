#![crate_name = "rustic"]
#![crate_version = "0.0.0"]
#![allow(ctypes)]
#![feature(globs)]
#![feature(lang_items)]
#![crate_type="lib"]
#![no_std]

extern crate core;

#[allow(dead_code)]
#[path = "arch/x86/"]
mod platform {
  pub mod drivers;
  pub mod io;
}

pub mod kernel;

#[lang = "begin_unwind"]
extern fn begin_unwind(args: &core::fmt::Arguments, file: &str, line: uint) -> ! {
  loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
