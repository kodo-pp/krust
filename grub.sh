#!/usr/bin/env bash
set -euo pipefail
./build.sh
cd build
rm -rf iso
mkdir -p iso
cp -r ../buildsystem/grub/boot iso/
cp krust iso/boot/grub/
grub-mkrescue iso -o krust.iso
