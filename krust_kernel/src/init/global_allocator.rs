use core::fmt::Write;
use linked_list_allocator::LockedHeap;

extern "C" {
    static _start_kheap_mem: u8;
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the kernel's global allocator.
/// SAFETY: must be called at most once.
pub unsafe fn init_global_allocator() {
    let mem_start: *const u8 = &_start_kheap_mem;
    // Hardcode 64 MiB at first.
    let mem_size = 64 << 20;
    log_raw!(
        "Kernel heap layout: start at {:#p}, size {} MiB",
        mem_start,
        mem_size >> 20,
    );
    GLOBAL_ALLOCATOR = LockedHeap::new(mem_start as usize, mem_size);
}

