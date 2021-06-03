#!/usr/bin/env bash
killall qemu-system-riscv64
nohup bash -c "make run > run.log 2>&1" &
echo "Done!"
