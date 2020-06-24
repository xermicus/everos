#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vgabuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExit {
    Success = 0x10,
    Failed = 0x11,
}

impl QemuExit {
    pub fn bb(&self) {
        use x86_64::instructions::port::Port;
        // SAFETY "writing to an I/O port can generally result in arbitrary behavior"
        unsafe {
            let mut port = Port::new(0xf4);
            port.write(*self as u32);
        }
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    // SAFETY undefined behavior if the PIC is misconfigured
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    QemuExit::Success.bb();
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    QemuExit::Failed.bb();
    hlt_loop()
}

#[cfg(test)]
entry_point!(test_kernel_main);
/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
