use platform::drivers::vga;
use platform::io;

#[no_mangle]
pub fn kmain() -> i32 {
  io::write("olar.meu.amigo");
  0
}

