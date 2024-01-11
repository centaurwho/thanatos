#![no_std]
#![no_main]

use core::panic::PanicInfo;

// TODO: Add a README.md file containing how to build and run the kernel

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}
