/* SPDX-License-Identifier: GPL-3.0-or-later */
/* Copyright (c) 2021 Alexander Korzun. */

ENTRY(_start)

SECTIONS {
    . = 1024 * 1024;

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
        *(.bss)
    }

    .layout : ALIGN(4096) {
        _start_usable_mem = .;
        . = . + 1024 * 1024;
        _end_kstack_mem = .;
        /* Skip 16 pages to make sure the stack doesn't interfere with the heap. */
        . = . + 4096 * 16;
        _start_kheap_mem = .;
    }
}
