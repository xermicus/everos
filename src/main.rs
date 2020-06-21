#![no_std]
#![no_main]

mod vgabuf;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!\n{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", 42);
    loop {}
}
