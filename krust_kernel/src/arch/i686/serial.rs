use super::Port;
use core::fmt::{self, Display, Formatter};
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SerialInitError {
    pub port: Port,
}

impl Display for SerialInitError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "Failed to initialize serial port at I/O port {:#x}",
            self.port.0
        )
    }
}

/// A serial port available at a specific I/O port.
#[derive(Debug)]
pub struct Serial {
    port: Port,
}

impl Serial {
    /// Initialize the serial port at the given I/O port.
    /// If the initialization probe fails, returns an error.
    /// SAFETY: the I/O port must be valid for a serial port AND `initialize`
    /// must not be called more than once for any I/O port number.
    unsafe fn try_initialize(port: Port) -> Result<Self, SerialInitError> {
        let reg_divisor_lsb = port.offset(0);
        let reg_divisor_msb = port.offset(1);
        let reg_line_control = port.offset(3);
        let reg_scratch = port.offset(7);

        // Perform the probe.
        if !probe(reg_scratch) {
            return Err(SerialInitError { port });
        }

        // Set mode: 8 bits/char, no parity, 1 stop bit, DLAB on.
        const BASE_FLAGS: u8 = 0b_0000_0011;
        const DLAB_FLAG: u8 = 0b_1000_0000;
        reg_line_control.out8(BASE_FLAGS | DLAB_FLAG);

        // Set divisor = 1 (baud rate = 115200);
        reg_divisor_lsb.out8(1);
        reg_divisor_msb.out8(0);

        // Turn DLAB off.
        reg_line_control.out8(BASE_FLAGS);

        // We're done.
        Ok(Self { port })
    }

    /// Write a byte to the serial port.
    #[inline(always)]
    pub fn write_byte(&self, b: u8) {
        // SAFETY: the port probe has been verified to belong to a COM port.
        // The verification is not 100% precise, but that's better than nothing.
        // And after all, aren't we up for some bad things if we're writing
        // an OS kernel?)
        unsafe {
            self.port.out8(b);
        }
    }

    /// Write a character to the serial port.
    pub fn write_char(&self, c: char) {
        let mut buf = [0; 4];
        c.encode_utf8(&mut buf);
        let num_bytes = c.len_utf8();
        for &b in &buf[..num_bytes] {
            self.write_byte(b);
        }
    }

    /// Write a string to the serial port.
    pub fn write_string(&self, s: &str) {
        // TODO: maybe more efficient (buffered) utf-8 encoding?
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

unsafe fn probe(reg_scratch: Port) -> bool {
    let initial_data = reg_scratch.in8();
    let modified_data = !initial_data;
    assert_ne!(initial_data, modified_data);
    reg_scratch.out8(modified_data);
    let new_data = reg_scratch.in8();
    modified_data == new_data
}

lazy_static! {
    pub static ref COM1: Serial = unsafe {
        Serial::try_initialize(Port(0x3F8)).expect("COM1 initialization failed")
    };
}
