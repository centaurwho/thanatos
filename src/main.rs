#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(thanatos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use thanatos::println;

// TODO: Add a README.md file containing how to build and run the kernel

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    thanatos::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    thanatos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}
