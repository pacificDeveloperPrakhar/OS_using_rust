// adding the inline assembly code ,asm macro will help us
// #![feature(asm)]
// this macro tells to eclude oall the standard library that are imported by default in the crate
#![no_std]
// also no main function we do be creating our won custom startup rather than crt0
#![no_main]
// importing the necessary testing rlated packages
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
// now we gonna import crate which provide the macro to wirte  embedded assembly code
// This feature is needed for `const` in asm!
#![feature(asm_const)]
// now import the asm code
use core::arch::asm;
// now import the x86_64 crate
use x86_64;
// this macro tells to create the new main test harness so that we can call it inside of the code
pub mod vga_buffer;
pub mod serial;

use core::panic::PanicInfo;
// this ttribute adds metadata to the function telling to execute ehen it encounters the panic
#[cfg(not(test))]
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
  // here i
  use os_rust::interrupt::init;
  init();
  // cause a kernel stack overflow by calling a function recursively
  recursively_call_fn();
     // invoke a breakpoint exception
  // x86_64::instructions::interrupts::int3(); 
  // ===========================invoking the page fault exception
  unsafe {
    *(0xdeadbeef as *mut u8) = 42;
  };

 
  // ==========================================================================================================
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
  // comment out this later when in  production
  // exit_qemu(QemuExitCode::Success);
 loop
 {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    
}

// our custom test runner
// so Fn() is the trait that takes the collection of reference of types that can be called as function
// &[] collection of references
// & dyn Fn() refernce to the type implemneting the Fn() trait

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    // ==================================================================================================
    // we gonna be using the serial_println! to print to the host

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
//    unsafe{
//     *memory.offset(0)=0x10;
// };
// unsafe{

//   ptr::write_volatile(memory,33);
// }
// use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }

unsafe {
  // The `asm!` macro takes the assembly code string and a list of operands.
  // `out dx, al` is the instruction.
  // `in("al") value` tells the compiler to place the `value` variable into the `al` register.
  // `in("dx") port` tells the compiler to place the `port` variable into the `dx` register.
  // `options(nostack, nomem)` are optimizations that inform the compiler
  // that the assembly code doesn't modify the stack or memory.
  asm!(
      "out dx, al",
      in("dx") 0xf4 as u16,
      in("al") exit_code as i8,
      options(nostack, nomem)
  );
}
}

#[cfg(test)]
#[panic_handler]
pub fn on_panic_encounter_during_test(info:&PanicInfo)->!
{
  serial_println!("[failed]\n");
  serial_println!("error encountered during test: {}",info);
  exit_qemu(QemuExitCode::Success);
  loop{}
}



#[test_case]
pub fn if_exception_breakpoint_works()
{
  unsafe{
    asm!(
      "int3"
    );
  };
}

pub fn recursively_call_fn()
{
  recursively_call_fn();
}