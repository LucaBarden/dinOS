#![no_std] // Dont link the Rust std library
#![no_main] // disable rust entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler] // this function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // since the linker looks for a function called _start(), this is our new entry point

    println!("Hello World{}", "!");
    loop {}
}
