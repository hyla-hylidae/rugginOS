#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printkln!("{}", info);
    loop{}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {

    printkln!("Hello, World!");

    #[cfg(test)]
    test_main();

    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    printkln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    printk!("trivial assertion... ");
    assert_eq!(1,1);
    printkln!("[ok]");
}
