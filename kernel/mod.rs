use platform::drivers::vga;
use platform::io;

#[start]
#[no_mangle]
pub unsafe fn main(_argc: int, _argv: *mut u8) -> int {
  vga::clear_screen(vga::Black);
  io::write("olarmundo");
  0
}
