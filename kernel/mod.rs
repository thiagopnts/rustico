use platform::drivers::vga;
use platform::io;

#[start]
#[no_mangle]
pub unsafe fn kmain() {
  vga::clear_screen(vga::Black);
  io::write("olar");
}
