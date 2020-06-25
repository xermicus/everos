#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]

extern crate alloc;
#[cfg(test)]
use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use memory::BootInfoFrameAllocator;
use x86_64::{structures::paging::OffsetPageTable, VirtAddr};

pub mod allocator;
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

pub fn create_heap(
    boot_info: &'static BootInfo,
) -> (OffsetPageTable<'_>, memory::BootInfoFrameAllocator) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    (mapper, frame_allocator)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
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
