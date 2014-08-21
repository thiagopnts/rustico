use platform::drivers::vga;
use platform::io;

#[no_mangle]
pub fn kmain() -> int {
  io::write("olar");
  0
}

