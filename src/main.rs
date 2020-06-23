#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(everos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use everos::{print_panic, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    everos::init();

    unsafe {
        println!("{}", *(0x2037d1 as *mut u32));
    }
    x86_64::instructions::interrupts::int3();
    println!("Hello World!");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    #[cfg(test)]
    test_main();
    everos::hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_panic!("KERNEL PANIC!\n{}", info);
    everos::hlt_loop()
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
