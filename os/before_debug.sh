#!/usr/bin/env bash
killall qemu-system-riscv64
make kernel
nohup bash -c "make run > run.log 2>&1" &
echo "Done!"
