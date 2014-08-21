#![crate_name = "rustic"]
#![crate_type = "lib"]
#![allow(ctypes)]
#![feature(globs)]
#![no_std]
extern crate core;

use platform::io;

#[path = "arch/x86/"]
mod platform {
  pub mod drivers;
  pub mod io;
}

pub mod kernel;
