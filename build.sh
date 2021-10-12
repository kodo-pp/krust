#!/usr/bin/env bash
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
    -Z build-std=core
cp "target/rustc-target/${KRUST_BUILDTYPE}/libkrust_kernel.a" build/
echo '... done' >&2

echo 'Building boot modules... ' >&2
nasm -f elf -o build/boot.o "boot/${KRUST_TARGET}/boot.asm"
echo '... done' >&2

echo 'Linking kernel... ' >&2
ld.lld -T "buildsystem/${KRUST_TARGET}/link.ld" build/boot.o -L build/ -lkrust_kernel -o build/krust
if [[ $do_strip = 1 ]]; then
    llvm-strip --strip-all build/krust
fi
du -sh build/krust
echo '... done' >&2

echo >&2
echo 'Done building kernel' >&2
