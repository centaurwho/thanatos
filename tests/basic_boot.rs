#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(thanatos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use thanatos::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    thanatos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
