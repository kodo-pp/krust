mod global_allocator;

use core::sync::atomic::{AtomicBool, Ordering};

/// Perform the global initialization work.
/// SAFETY: must be called at most once.
unsafe fn init_all() {
    global_allocator::init_global_allocator();
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
