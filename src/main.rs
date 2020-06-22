#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(everos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use everos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    everos::init();

    x86_64::instructions::interrupts::int3();
    println!("Hello World! {}", 42);

    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!\n{}", info);
    loop {}
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
