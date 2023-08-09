// Defines the Global Descriptor Table and the Task State Segment

// The GDT used to be used to store segments, before paging was common
// Now it is used for two things:
// - To store the TSS
// - To switch between kernel and user mode

use lazy_static::lazy_static;
use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS, DS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

lazy_static! {
    static ref TSS: TaskStateSegment = {
        // The TaskStateSegment used to hold processor register state about a task in 32-bit mode
        // Now it contains two stack tables (the Privilege Stack Table and the Interrupt Stack Table)
        // It also contains the I/O Map Base Address
        let mut tss = TaskStateSegment::new();

        // Store the double fault interrupt stack in the appropriate index of the interrupt stack table
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // The double fault stack is 5 pages long

            // We don't have memory management :(
            // This syntax means fill it with a bunch of 0s (u8)
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // Get the virtual address of the stack
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;

            // x86 stacks grow downwards so we write the high address to the stack table
            stack_end
        };
        tss
    };
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment()); // See: https://github.com/rust-osdev/bootloader/issues/196
        (gdt, Selectors { code_selector, data_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
        DS::set_reg(GDT.1.data_selector);
    }
}
