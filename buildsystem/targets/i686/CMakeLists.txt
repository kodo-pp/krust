# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.

set(KRUST_TARGET_TRIPLET i686-elf)
list(APPEND KRUST_TOOLCHAIN_TOOLS cargo ld nasm strip)

if ("${KRUST_ENABLE_GRUB_ISO}")
    list(APPEND KRUST_TOOLCHAIN_TOOLS grub_mkrescue)
endif ()
