#![no_std] // Dont link the Rust std library
#![no_main] // disable rust entry points
#![feature(custom_test_frameworks)]
#![test_runner(din_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use din_os::println;

#[no_mangle] // dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    // since the linker looks for a function called _start(), this is our new entry point

    println!("Hello World{}", "!");
    din_os::init();

    #[cfg(test)]
    test_main();

    din_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler] // this function is called on panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    din_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    din_os::test_panic_handler(info);
}
