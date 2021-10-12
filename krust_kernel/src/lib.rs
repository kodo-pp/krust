#![no_std]
#![feature(asm)]
#![feature(once_cell)]
#![feature(panic_info_message)]

mod arch;
mod log;
mod runtime;

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
    panic!("Test panic message");
}
