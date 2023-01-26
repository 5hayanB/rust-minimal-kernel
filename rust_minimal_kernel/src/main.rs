#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_minimal_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use rust_minimal_kernel::println;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_minimal_kernel::test_panic_handler(info)
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
