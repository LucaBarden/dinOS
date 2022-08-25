#![no_std] // Dont link the Rust std library
#![no_main] // disable rust entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler] // this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // since the linker looks for a function called _start(), this is our new entry point

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    
    loop {}
}
