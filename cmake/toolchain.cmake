# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.

function(krust_find_cargo)
    message("Looking for cargo...")
    find_program(CARGO cargo)
    if ("${CARGO}" STREQUAL CARGO-NOTFOUND)
        message(
            FATAL_ERROR
            "Cargo was not found. "
            "You need a Rust + Cargo (via rustup) installation to build Krust."
        )
    endif ()

    execute_process(
        COMMAND "${CARGO}" +nightly --version
        TIMEOUT 5
        RESULT_VARIABLE CARGO_VERSION_RESULT
        OUTPUT_VARIABLE CARGO_VERSION_OUTPUT
    )
    if (NOT "${CARGO_VERSION_RESULT}" STREQUAL 0)
        message(
            FATAL_ERROR
            "Failed to run `cargo +nightly --version`. "
            "Check that Rust has been installed via rustup."
        )
    endif ()
    if (NOT "${CARGO_VERSION_OUTPUT}" MATCHES "nightly")
        message(
            FATAL_ERROR
            "Cargo doesn't seem to support a nightly version. "
            "Check that Rust nightly has been installed via rustup."
        )
    endif ()

    string(STRIP "${CARGO_VERSION_OUTPUT}" CARGO_PRETTY_VERSION)
    message("Found ${CARGO} (${CARGO_PRETTY_VERSION}).")
    set(KRUST_TOOL_CARGO "${CARGO}" PARENT_SCOPE)
endfunction()

function(krust_find_ld)
    message("Looking for ld...")
    find_program(GNU_CROSS_LD "${KRUST_TARGET_TRIPLET}-ld")
    if (NOT "${GNU_CROSS_LD}" STREQUAL GNU_CROSS_LD-NOTFOUND)
        message("Found GNU ld (for ${KRUST_TARGET_TRIPLET}) ${GNU_CROSS_LD}.")
        set(KRUST_TOOL_LD "${GNU_CROSS_LD}" PARENT_SCOPE)
        return()
    endif ()

    message(WARNING "GNU ld (for ${KRUST_TARGET_TRIPLET}) not found. Will try to use LLVM lld.")
    find_program(LLD ld.lld)
    if (NOT "${LLD}" STREQUAL LLD-NOTFOUND)
        message(
            WARNING
            "Found LLVM lld ${LLD}. This will result in larger binaries. "
            "Install GNU ld (for ${KRUST_TARGET_TRIPLET}) for `--gc-sections` to work better."
        )
        set(KRUST_TOOL_LD "${LLD}" PARENT_SCOPE)
        return()
    endif ()

    message(
        FATAL_ERROR
        "No suitable linker found. Install either GNU ld (for ${KRUST_TARGET_TRIPLET}), "
        "or LLVM lld."
    )
endfunction()

function(krust_find_nasm)
    message("Looking for nasm...")
    find_program(NASM nasm)
    if ("${NASM}" STREQUAL NASM-NOTFOUND)
        message(
            FATAL_ERROR
            "Nasm was not found. "
            "You need nasm to build Krust for ${KRUST_TARGET}."
        )
    endif ()
    
    message("Found ${NASM}.")
    set(KRUST_TOOL_NASM "${NASM}" PARENT_SCOPE)
endfunction()

function(krust_find_strip)
    message("Looking for strip...")
    find_program(GNU_CROSS_STRIP "${KRUST_TARGET_TRIPLET}-strip")
    if (NOT "${GNU_CROSS_STRIP}" STREQUAL GNU_CROSS_STRIP-NOTFOUND)
        message("Found GNU strip (for ${KRUST_TARGET_TRIPLET}) ${GNU_CROSS_STRIP}.")
        set(KRUST_TOOL_STRIP "${GNU_CROSS_STRIP}" PARENT_SCOPE)
        return()
    endif ()

    message("GNU strip (for ${KRUST_TARGET_TRIPLET}) not found. Will try to use LLVM strip.")
    find_program(LLVM_STRIP llvm-strip)
    if (NOT "${LLVM_STRIP}" STREQUAL LLVM_STRIP-NOTFOUND)
        message("Found LLVM strip ${LLVM_STRIP}.")
        set(KRUST_TOOL_STRIP "${LLVM_STRIP}" PARENT_SCOPE)
        return()
    endif ()

    message(
        FATAL_ERROR
        "No suitable linker found. Install either GNU strip (for ${KRUST_TARGET_TRIPLET}), "
        "or LLVM strip."
    )
endfunction()

function(krust_find_grub_mkrescue)
    message("Looking for grub-mkrescue...")
    find_program(GRUB_MKRESCUE grub-mkrescue)
    if ("${GRUB_MKRESCUE}" STREQUAL GRUB_MKRESCUE-NOTFOUND)
        message(
            FATAL_ERROR
            "grub-mkrescue was not found. "
            "You need it to build a bootable ISO image."
        )
    endif ()
    
    message("Found ${GRUB_MKRESCUE}.")
    set(KRUST_TOOL_GRUB_MKRESCUE "${GRUB_MKRESCUE}" PARENT_SCOPE)
endfunction()

foreach (TOOL ${KRUST_TOOLCHAIN_TOOLS})
    cmake_language(CALL "krust_find_${TOOL}")
endforeach ()
