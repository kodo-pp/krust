#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.
set -euo pipefail
KRUST_TARGET="${KRUST_TARGET:-i686}"
KRUST_BUILDTYPE="${KRUST_BUILDTYPE:-release}"

cargo_opts=()
do_strip=0
case $KRUST_BUILDTYPE in
    (release)
        cargo_opts+=(--release)
        #do_strip=1
        ;;
    (*)
        ;;
esac

mkdir -p build

echo 'Building kernel... ' >&2
cargo +nightly build \
    --target="buildsystem/${KRUST_TARGET}/rustc-target.json" \
    "${cargo_opts[@]}" \
    -Z build-std=core,compiler_builtins \
    -Z build-std-features=compiler-builtins-mem
cp "target/rustc-target/${KRUST_BUILDTYPE}/libkrust_kernel.a" build/
echo '... done' >&2

echo 'Building boot modules... ' >&2
nasm -f elf -o build/boot.o "boot/${KRUST_TARGET}/boot.asm"
echo '... done' >&2

echo 'Linking kernel... ' >&2
ld.lld \
    -T "buildsystem/${KRUST_TARGET}/link.ld" \
    -o build/krust \
    --gc-sections \
    build/boot.o build/libkrust_kernel.a
if [[ $do_strip = 1 ]]; then
    llvm-strip --strip-all build/krust
fi
du -sh build/krust
echo '... done' >&2

echo >&2
echo 'Done building kernel' >&2
