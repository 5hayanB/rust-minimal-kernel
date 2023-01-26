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
    rust_minimal_kernel::hlt_loop();
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_minimal_kernel::init();
    //x86_64::instructions::interrupts::int3();  // Breakpoint Exception

    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;  // Double Fault
    //};

    //fn stack_overflow() {  // Kernel Stack Overflow -> Double Fault
    //    stack_overflow();
    //}
    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_minimal_kernel::hlt_loop();

    //loop {
    //    use rust_minimal_kernel::print;
    //    print!("-");
    //    for _ in 0..10000 {}  // Delay for the timer to print its dots
    //}
}


// Test Section
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_minimal_kernel::test_panic_handler(info)
}
