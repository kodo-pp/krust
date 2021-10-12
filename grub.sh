#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (c) 2021 Alexander Korzun.
set -euo pipefail
./build.sh
cd build
rm -rf iso
mkdir -p iso
cp -r ../buildsystem/grub/boot iso/
cp krust iso/boot/grub/
grub-mkrescue iso -o krust.iso
