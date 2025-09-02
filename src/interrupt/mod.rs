use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;

// static struct object of the idt

// lazy_static! doesn’t directly allocate on the heap.

// It generates a hidden static that holds the Mutex<Option<T>> (or similar), initialized at compile time.

// When you first use SERIAL1, it checks if the value is initialized. If not, it runs your closure once and stores the result inside that static slot.

// Since you don’t have a heap allocator, the memory for SERIAL1 still comes from static memory, not the heap.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // now we are setting all the interrupt handling over here while defining the idt
        idt.breakpoint.set_handler_fn(breakpoint_handle);
        idt
    };
}
// ======================================================================================================
pub fn init_idt()
{
    IDT.load();
}

// so we gonna create the handler interrupt for the breakpoint 
// x86-interrupt tells that the callign convention are as according to
// the interrupt as the interrupt are handled by the cpu as
// the calling convention of interrupt and function are different
use crate::println;
pub extern "x86-interrupt" fn breakpoint_handle(stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// now initialize the idt
pub fn init()
{
    init_idt();
}