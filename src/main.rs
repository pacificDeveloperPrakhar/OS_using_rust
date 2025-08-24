// this macro tells to eclude oall the standard library that are imported by default in the crate
#![no_std]
// also no main function we do be creating our won custom startup rather than crt0
#![no_main]
use core::panic::PanicInfo;
// this ttribute adds metadata to the function telling to execute ehen it encounters the panic
#[panic_handler]
fn on_panic_encounter(panic:&PanicInfo)->!
{
  loop
  {

  }
}

