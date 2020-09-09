#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os_t::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;

use blog_os_t::println;
use core::panic::PanicInfo;
use x86_64;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hogehogeh");

    blog_os_t::init();
    #[cfg(test)]
    test_main();
    blog_os_t::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    blog_os_t::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os_t::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
