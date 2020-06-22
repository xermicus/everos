#![no_std]
#![no_main]

use core::panic::PanicInfo;
use everos::{serial_print, serial_println, QemuExit};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    QemuExit::Failed.bb();
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    QemuExit::Success.bb();
    loop {}
}
