ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .multiboot : ALIGN(4096) {
        *(.multiboot)
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
    _start_kstack_mem = .;
    . = . + 0x100000;
    _start_kheap_mem = .;
}
