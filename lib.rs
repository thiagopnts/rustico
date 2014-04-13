#![crate_id = "rustic#0.0"]
#![allow(ctypes)]
#![no_std]

extern crate core;

#[allow(dead_code)]
#[path = "arch/x86/"]
mod platform {
  pub mod drivers;
}

pub mod kernel;

