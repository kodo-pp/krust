# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.

add_custom_command(
    OUTPUT libkrust_kernel.a
    COMMAND "${KRUST_TOOL_CARGO}" +nightly build --target=${KRUST_TARGET_JSON} ${KRUST_CARGO_FLAGS}
    COMMAND cp
        "${KRUST_TARGET_CARGO_DIR}/${KRUST_BUILDTYPE}/libkrust_kernel.a"
        "${CMAKE_CURRENT_BINARY_DIR}/"
    DEPFILE "${KRUST_TARGET_CARGO_DIR}/${KRUST_BUILDTYPE}/libkrust_kernel.d"
    COMMENT "Compiling the kernel"
    VERBATIM
    USES_TERMINAL
)
