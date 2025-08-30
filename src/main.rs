// adding the inline assembly code ,asm macro will help us
// #![feature(asm)]
// this macro tells to eclude oall the standard library that are imported by default in the crate
#![no_std]
// also no main function we do be creating our won custom startup rather than crt0
#![no_main]


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
  panic!("lets generate the panic to see how  the panic handler respinds");
  
 loop
 {}
}