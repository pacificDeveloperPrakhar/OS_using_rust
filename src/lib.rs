#![no_std]
// also no main function we do be creating our won custom startup rather than crt0
#![no_main]

// now to support the x86-interrupt calling convention
#![feature(abi_x86_interrupt)]
// creating the modules for each functionality ,importing them
pub mod vga_buffer;
pub mod serial;
pub mod interrupt;