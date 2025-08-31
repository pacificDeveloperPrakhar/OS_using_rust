// adding the inline assembly code ,asm macro will help us
// #![feature(asm)]
// this macro tells to eclude oall the standard library that are imported by default in the crate
#![no_std]
// also no main function we do be creating our won custom startup rather than crt0
#![no_main]
// importing the necessary testing rlated packages
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// this macro tells to create the new main test harness so that we can call it inside of the code
#![reexport_test_harness_main = "test_main"]

pub mod vga_buffer;

use core::panic::PanicInfo;
// this ttribute adds metadata to the function telling to execute ehen it encounters the panic
#[panic_handler]
fn on_panic_encounter(panic:&PanicInfo)->!
{
  println!("{}", panic);
  loop
  {

  }
}
// this attribute tells to not use the hex name cryptic for the function rather we want the orignal name
// we will be usign the c s calling convention instead of the rust calling convention
static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start()->!
{
  // let vga_buffer = 0xb8000 as *mut u8;

  // for (i, &byte) in HELLO.iter().enumerate() {
  //     unsafe {
  //         *vga_buffer.offset(i as isize * 2) = byte;
  //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
  //     }
  // }
  // crate::vga_buffer::write_something();
  // panic!("lets generate the panic to see how  the panic handler respinds");
  // this macro tells to not include while cargo build and include only while testing using the cargo test
  #[cfg(test)]
  test_main();
 loop
 {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

// our custom test runner
// so Fn() is the trait that takes the collection of reference of types that can be called as function
// &[] collection of references
// & dyn Fn() refernce to the type implemneting the Fn() trait

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
