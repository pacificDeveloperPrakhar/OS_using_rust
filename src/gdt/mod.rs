use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_1ST_INDEX:u16=0;
lazy_static!
{

    static ref TSS:TaskStateSegment={
        let tss=TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_1ST_INDEX]={
            // one memory page=4096
            // so 5 pages are allocated
            let stack_size=4096*5;
            // creating a static mutable array to be used as the stack since we dont ahve a heap
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            &raw const STACK

            // &STACK → normally creates a safe Rust reference (&[u8; STACK_SIZE]).

            // &raw const STACK → creates a raw pointer (*const [u8; STACK_SIZE]) instead.

            // Why raw? The CPU just needs an address, not a Rust reference with lifetimes or borrowing checks.

            // This avoids Rust’s safety checks, which are unnecessary for low-level stack setup.
            let stack_start=VirtAddr::fromPtr(&raw const STACK);

            // VirtAddr = a wrapper around a 64-bit virtual address (used by the x86_64 crate).

            // from_ptr converts the raw pointer into a VirtAddr object.

            // This is needed because the TSS and IST use virtual addresses, not Rust references.
            let stack_end = stack_start + STACK_SIZE;
            // now we store the end address of the stack since stack is starting from the bottom
            // We write the top address because stacks on x86 grow downwards, i.e., from high addresses to low addresses.
            return stack_end;
        };
        return tss;
    }
}