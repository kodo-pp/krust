// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2021 Alexander Korzun.
use core::convert::TryInto;

/// An I/O port handle. A thin wrapper around the underlying port number for type safety and
/// convenience.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Port(pub u16);

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> u16 {
        port.0
    }
}

impl Port {
    /// Obtain
    pub fn offset(self, offset: i16) -> Self {
        let resulting_i32 = self.0 as i32 + offset as i32;
        let resulting_u16: u16 = resulting_i32
            .try_into()
            .expect("I/O port number was offset beyond the valid port number range");
        resulting_u16.into()
    }

    #[inline(always)]
    pub unsafe fn in8(self) -> u8 {
        let result: u8;
        asm!("in al, dx", in("dx") self.0, out("al") result);
        result
    }

    #[inline(always)]
    pub unsafe fn in16(self) -> u16 {
        let result: u16;
        asm!("in ax, dx", in("dx") self.0, out("ax") result);
        result
    }

    #[inline(always)]
    pub unsafe fn in32(self) -> u32 {
        let result: u32;
        asm!("in eax, dx", in("dx") self.0, out("eax") result);
        result
    }

    #[inline(always)]
    pub unsafe fn out8(self, data: u8) {
        asm!("out dx, al", in("dx") self.0, in("al") data);
    }

    #[inline(always)]
    pub unsafe fn out16(self, data: u16) {
        asm!("out dx, ax", in("dx") self.0, in("ax") data);
    }

    #[inline(always)]
    pub unsafe fn out32(self, data: u32) {
        asm!("out dx, eax", in("dx") self.0, in("eax") data);
    }
}
