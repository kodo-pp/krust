/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Copyright (c) 2021 Alexander Korzun. */

ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .multiboot : ALIGN(4096) {
        KEEP(*(.multiboot))
    }
    .text : ALIGN(4096) {
        *(.text)
    }
    .data : ALIGN(4096) {
        *(.data)
    }
    .rodata : ALIGN(4096) {
        *(.rodata)
    }
    .bss : ALIGN(4096) {
        *(COMMON)
    }

    . = (. + 4095) & ~4095;
    _start_usable_mem = .;
    . = . + 0x100000;
    _end_kstack_mem = .;
    _start_kheap_mem = .;
}
