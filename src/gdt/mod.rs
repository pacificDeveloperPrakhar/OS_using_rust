use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::Descriptor;

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


lazy_static!
{
    pub static ref GDT:GlobalDescriptorTable={
     let gdt=GlobalDescriptorTable::new();
//      Descriptor::kernel_code_segment() creates a descriptor for kernel-mode code:

// Executable code that runs at privilege level 0 (ring 0).

// Typically read-only (cannot write to code memory).

// Used by the CPU to know:

// Base address of the segment (usually 0 in flat mode)

// Segment limit (size)

// Privilege level (kernel vs user)

// Segment type (code, executable, readable)
     gdt.add_entry(Descriptor::kernel_code_segment());
// adding our defined task state segment into the gdt
     gdt.add_entry(Descriptor::tss_segment(&TSS));
     return gdt;
    };
    return GDT;
}


// this fn will load the gdt
pub fn init_gdt()
{
    GDT.load()
}