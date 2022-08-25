#![no_std] // Dont link the Rust std library
#![no_main] // disable rust entry points

use core::panic::PanicInfo;

#[panic_handler] // this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";


#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // since the linker looks for a function called _start(), this is our new entry point
    
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
