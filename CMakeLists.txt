# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.

cmake_minimum_required(VERSION 3.7)
project(krust)

set(KRUST_TARGET CACHE STRING "Target to build Krust for")
if ("${KRUST_TARGET}" STREQUAL "")
    message(FATAL_ERROR "KRUST_TARGET must be set")
endif ()

option(KRUST_ENABLE_GRUB_ISO OFF "Build a bootable ISO image (with GRUB)")

set(KRUST_CARGO_FLAGS)
if ("${CMAKE_BUILD_TYPE}" STREQUAL Release)
    set(KRUST_BUILDTYPE release)
    list(APPEND KRUST_CARGO_FLAGS --release)
else ()
    set(KRUST_BUILDTYPE debug)
endif ()

set(KRUST_TARGET_BUILDSYSTEM_DIR "${CMAKE_SOURCE_DIR}/buildsystem/targets/${KRUST_TARGET}")
set(KRUST_TARGET_BOOT_DIR "${CMAKE_SOURCE_DIR}/boot/${KRUST_TARGET}")
set(KRUST_TARGET_JSON "${KRUST_TARGET_BUILDSYSTEM_DIR}/krust-${KRUST_TARGET}.json")
set(KRUST_TARGET_CARGO_DIR "${CMAKE_SOURCE_DIR}/target/krust-${KRUST_TARGET}")

include("${KRUST_TARGET_BUILDSYSTEM_DIR}/CMakeLists.txt")
include(cmake/toolchain.cmake)
include(krust_kernel/CMakeLists.txt)
include("${KRUST_TARGET_BOOT_DIR}/CMakeLists.txt")
