use core::cmp::Ordering;
use core::panic::PanicInfo;
use crate::log::default_logger;
use core::fmt::{self, Write};

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let mut logger = default_logger();
    let _ = logger.write_str("Kernel panic: ");
    let _ = logger.write_str("<> ");
    logger.write_fmt(format_args!("{:#p}", panic_info));
    //fmt::write(&mut logger, format_args!("{:#p}", panic_info));
    let maybe_args = panic_info.message();
    let _ = logger.write_str("(got args) ");
    if let Some(args) = maybe_args {
        let _ = logger.write_str("[");
        logger.write_fmt(format_args!("{:#p}", args));
        let args_copy = *args;
        logger.write_str("(got args copy)");
        logger.write_fmt(format_args!("{:#p}", &args_copy));
        let _ = logger.write_str("]\n");
    } else {
        let _ = logger.write_str("Unknown panic message\n");
    }

    loop {
        unsafe { asm!("cli", "hlt") };
    }
}

#[no_mangle]
unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        dest.add(i).write_volatile(src.add(i).read_volatile());
    }
    dest
}

#[no_mangle]
unsafe fn memset(dest: *mut u8, c_fill: i32, n: usize) -> *mut u8 {
    let fill = c_fill as u8;
    for i in 0..n {
        dest.add(i).write_volatile(fill);
    }
    dest
}

#[no_mangle]
unsafe fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let value1 = src1.add(i).read_volatile();
        let value2 = src2.add(i).read_volatile();
        match value1.cmp(&value2) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            Ordering::Equal => (),
        }
    }
    0
}
