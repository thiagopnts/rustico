use platform::drivers::vga;

#[start]
#[no_mangle]
pub unsafe fn main() {
  vga::clear_screen(vga::Black);
  vga::write("OLAR");
}
