section .multiboot
multiboot.value.magic equ 0x1BADB002
multiboot.value.flags equ 0x00000000
multiboot.value.checksum equ -(multiboot.value.magic + multiboot.value.flags)
multiboot.magic: dd multiboot.value.magic
multiboot.flags: dd multiboot.value.flags
multiboot.checksum: dd multiboot.value.checksum

section .text
global _start
extern main
_start:
    call main
_start.hcf:
    cli
    hlt
    jmp _start.hcf
