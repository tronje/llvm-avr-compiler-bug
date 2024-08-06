#!/bin/bash


avrdude -V -P usb -c dragon_jtag -p m164p -U flash:w:target/avr-atmega164pa/release/llvm-avr-compiler-bug.elf:e -U lfuse:w:0xE7:m -U hfuse:w:0x98:m -U lock:w:0xEF:m -v
