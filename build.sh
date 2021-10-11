#!/usr/bin/env bash
set -euo pipefail
KRUST_TARGET="${KRUST_TARGET:-i686}"

mkdir -p build

echo 'Building kernel... ' >&2
cargo +nightly build --target="buildsystem/${KRUST_TARGET}/rustc-target.json" --release -Z build-std=core
cp target/rustc-target/release/libkrust_kernel.a build/
echo '... done' >&2

echo 'Building boot modules... ' >&2
nasm -f elf -o build/boot.o "boot/${KRUST_TARGET}/boot.asm"
echo '... done' >&2

echo 'Linking kernel... ' >&2
ld.lld -T "buildsystem/${KRUST_TARGET}/link.ld" build/boot.o -L build/ -lkrust_kernel -o build/krust --gc-sections --lto-O2
llvm-strip --strip-all build/krust
du -sh build/krust
echo '... done' >&2

echo >&2
echo 'Done building kernel' >&2
