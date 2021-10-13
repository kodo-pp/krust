# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.

add_custom_command(
    OUTPUT boot.o
    COMMAND "${KRUST_TOOL_NASM}"
        -f elf
        -o "${CMAKE_CURRENT_BINARY_DIR}/boot.o"
        "${CMAKE_CURRENT_SOURCE_DIR}/boot/i686/boot.asm"
    MAIN_DEPENDENCY boot/i686/boot.asm
    COMMENT "Assembling the boot entry point"
)

add_custom_command(
    OUTPUT krust
    COMMAND "${KRUST_TOOL_LD}"
        -T "${KRUST_TARGET_BUILDSYSTEM_DIR}/link.ld"
        -o "${CMAKE_CURRENT_BINARY_DIR}/krust"
        --gc-sections
        "${CMAKE_CURRENT_BINARY_DIR}/boot.o"
        "${CMAKE_CURRENT_BINARY_DIR}/libkrust_kernel.a"
    # TODO: skip stripping for debug builds.
    COMMAND "${KRUST_TOOL_STRIP}" "${CMAKE_CURRENT_BINARY_DIR}/krust" --strip-all
    DEPENDS
        "${CMAKE_CURRENT_BINARY_DIR}/boot.o"
        "${CMAKE_CURRENT_BINARY_DIR}/libkrust_kernel.a"
    COMMENT "Linking the kernel"
)
add_custom_target(image-kernel ALL DEPENDS krust)

if ("${KRUST_ENABLE_GRUB_ISO}")
    add_custom_command(
        OUTPUT krust.iso
        COMMAND rm -rf "${CMAKE_CURRENT_BINARY_DIR}/iso/"
        COMMAND mkdir -p "${CMAKE_CURRENT_BINARY_DIR}/iso/"
        COMMAND cp -rv buildsystem/grub/boot "${CMAKE_CURRENT_BINARY_DIR}/iso/"
        COMMAND cp "${CMAKE_CURRENT_BINARY_DIR}/krust" "${CMAKE_CURRENT_BINARY_DIR}/iso/boot/grub/krust"
        COMMAND "${KRUST_TOOL_GRUB_MKRESCUE}"
            -o "${CMAKE_CURRENT_BINARY_DIR}/krust.iso"
            "${CMAKE_CURRENT_BINARY_DIR}/iso"
        DEPENDS
            buildsystem/grub/boot/grub/grub.cfg
            "${CMAKE_CURRENT_BINARY_DIR}/krust"
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        COMMENT "Building a bootable ISO image"
    )
    add_custom_target(image-iso DEPENDS krust.iso)
endif ()
