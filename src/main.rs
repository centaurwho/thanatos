#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(thanatos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use thanatos::println;

// TODO: Add a README.md (build.rs would be even better) file containing how to build and run the kernel

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    thanatos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    thanatos::test_panic_handler(info)
}

entry_point!(kernel_main);

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    thanatos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    thanatos::hlt_loop();
}
