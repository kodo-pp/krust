// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2021 Alexander Korzun.
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    log_raw_nonl!("Kernel panic: ");
    if let Some(args) = panic_info.message() {
        log_raw!("{}", *args);
    } else {
        log_raw!("Unknown panic message");
    }

    loop {
        unsafe { asm!("cli", "hlt") };
    }
}
