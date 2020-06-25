#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(everos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use everos::{memory, print_panic, println};
use x86_64::{structures::paging::Page, VirtAddr};

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    everos::init();

    do_example_stuff(boot_info);

    #[cfg(test)]
    test_main();
    everos::hlt_loop()
}

#[doc(hidden)]
#[allow(unused)]
fn do_example_stuff(boot_info: &'static BootInfo) {
    // Print mapped virt addr pointing into kernel code
    unsafe {
        println!("{}", *(0x2037d1 as *mut u32));
    }
    x86_64::instructions::interrupts::int3();
    println!("Hello World!");

    // map an unused page
    let (mut mapper, mut frame_allocator) = everos::create_heap(boot_info);
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // Some allocation
    let heap_val = Box::new(42);
    println!("heap allocated value at {:p}", heap_val);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_64::instructions::interrupts::without_interrupts(|| {
        print_panic!("KERNEL PANIC!\n{}", info);
        everos::hlt_loop()
    })
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    everos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
