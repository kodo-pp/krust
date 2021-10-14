use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use linked_list_allocator::LockedHeap;

extern "C" {
    static _start_kheap_mem: u8;
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize the kernel's global allocator.
/// SAFETY: must be called at most once.
unsafe fn init_global_allocator() {
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

/// Perform the global initialization work.
/// SAFETY: must be called at most once.
unsafe fn init_all() {
    init_global_allocator();
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Perform the global initialization work. Panics if called more than once.
pub fn init_once() {
    // TODO: is `SeqCst` necessary?
    let initialized_before = INITIALIZED.swap(true, Ordering::SeqCst);
    if initialized_before {
        panic!("Attempted to call `init_once` more than once");
    }
    unsafe { init_all() };
}
