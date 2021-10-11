#![no_std]
use core::cmp::Ordering;
use core::panic::PanicInfo;

#[no_mangle]
unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *dest.add(i) = *src.add(i);
    }
    dest
}

#[no_mangle]
unsafe fn memset(dest: *mut u8, c_fill: i32, n: usize) -> *mut u8 {
    let fill = c_fill as u8;
    for i in 0..n {
        *dest.add(i) = fill;
    }
    dest
}

#[no_mangle]
unsafe fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let value1 = *src1.add(i);
        let value2 = *src2.add(i);
        match value1.cmp(&value2) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            Ordering::Equal => (),
        }
    }
    0
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
unsafe fn main() -> ! {
    let screen_out_ptr = 0xB8000 as *mut u8;
    let screen_out = core::slice::from_raw_parts_mut(screen_out_ptr, 32768);
    screen_out[0] = b'H';
    screen_out[1] = 0x07;
    screen_out[2] = b'e';
    screen_out[3] = 0x07;
    screen_out[4] = b'l';
    screen_out[5] = 0x07;
    screen_out[6] = b'l';
    screen_out[7] = 0x07;
    screen_out[8] = b'o';
    screen_out[9] = 0x07;
    panic!();
}
