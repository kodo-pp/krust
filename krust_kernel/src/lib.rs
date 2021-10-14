// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2021 Alexander Korzun.
#![no_std]
#![feature(asm)]
#![feature(once_cell)]
#![feature(panic_info_message)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

mod arch;
#[macro_use]
mod log;
mod init;
mod runtime;

use core::fmt::Write;

#[no_mangle]
unsafe fn main() -> ! {
    init::init_once();
    let vec = alloc::vec![1, 2, 3];
    log_raw!("{:?}", vec);
    panic!("Test panic message");
}
