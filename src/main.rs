#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(thanatos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

use thanatos::memory::BootInfoFrameAllocator;
use thanatos::{allocator, memory, println};

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

pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    thanatos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut frame_allocator, &mut mapper).expect("Heap initialization failed");

    let heap_value = Box::new(41);
    println!("Value at: {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    println!("Vec at {:p}", vec.as_slice());

    let ref_counted = Rc::new(vec![1, 2, 3]);
    let cloned_ref = ref_counted.clone();
    println!("Current ref count: {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(ref_counted);
    println!("Updated ref count: {}", Rc::strong_count(&cloned_ref));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    thanatos::hlt_loop();
}
